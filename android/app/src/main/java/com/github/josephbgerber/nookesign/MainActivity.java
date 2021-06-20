package com.github.josephbgerber.nookesign;

import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.content.SharedPreferences;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.os.BatteryManager;
import android.os.Bundle;
import android.os.Handler;
import android.view.View;
import android.widget.Button;
import android.widget.CheckBox;
import android.widget.EditText;
import android.widget.ImageView;
import android.widget.TextView;


import java.io.DataOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.MalformedURLException;
import java.net.URL;

import mjson.Json;


public class MainActivity extends Activity {

    String imageHash;

    @Override
    public void onCreate(Bundle bundle) {
        super.onCreate(bundle);
        setContentView(R.layout.activity_main);
    }

    @Override
    public void onStart() {
        super.onStart();

        final SharedPreferences preferences = getPreferences(Context.MODE_PRIVATE);

        int libraryId = preferences.getInt("libraryId", -1);
        final int deviceId = preferences.getInt("deviceId", -1);
        final String hostname = preferences.getString("hostname", null);
        final boolean disableAllInput = preferences.getBoolean("disableAllInput", false);

        if (libraryId != -1 && deviceId != -1 && hostname != null) {
            try {
                URL url = new URL("http://" + hostname);

                HttpURLConnection connection = (HttpURLConnection) (new URL(url, "device/" + deviceId).openConnection());
                connection.setRequestProperty("Accept-Charset", "UTF-8");

                int responseCode = connection.getResponseCode();

                if (responseCode == 403) {
                    // This device is not in the database. It may have been deleted. De-initialize this device.
                    SharedPreferences.Editor editor = preferences.edit();
                    editor.putInt("libraryId", -1);
                    editor.putInt("deviceId", -1);
                    editor.putString("hostname", null);
                    editor.putBoolean("disableAllInput", false);
                    editor.commit();
                }
                if (responseCode == 200) {
                    Json device = Json.read(readIntoString(connection.getInputStream()));
                    connection.disconnect();

                    if (device.isNull()) {
                        // This device is not in the database. It may have been deleted. De-initialize this device.
                        SharedPreferences.Editor editor = preferences.edit();
                        editor.putInt("libraryId", -1);
                        editor.putInt("deviceId", -1);
                        editor.putString("hostname", null);
                        editor.putBoolean("disableAllInput", false);
                        editor.commit();
                    } else {
                        setContentView(R.layout.activity_image);
                        if (disableAllInput) {
                            disableAllInput();
                        }
                        startUpdateTask();
                        return;
                    }
                }
            } catch (MalformedURLException e) {
                e.printStackTrace();
            } catch (IOException e) {
                e.printStackTrace();
            }
        }

        // This device is uninitialized. Ask the user for the necessary information to connect to the server.
        final TextView errors = (TextView) findViewById(R.id.errors);
        final EditText editHostname = (EditText) findViewById(R.id.editHostname);
        final EditText editLibraryId = (EditText) findViewById(R.id.editLibraryId);
        final CheckBox disableAllInputCheckbox = (CheckBox) findViewById(R.id.disable_all_input);

        System.out.println(editHostname == null);

        final Button button = (Button) findViewById(R.id.button);
        System.out.println(button == null);

        View.OnClickListener listener = new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                URL url;
                try {
                    url = new URL("http://" + editHostname.getText());
                } catch (MalformedURLException e) {
                    e.printStackTrace();
                    errors.setText(R.string.error_invalid_hostname);
                    errors.setVisibility(View.VISIBLE);
                    return;
                }

                String libraryName = editLibraryId.getText()
                        .toString()
                        .trim()
                        .toLowerCase()
                        .replace(' ', '-');

                Json library;

                try {
                    HttpURLConnection connection = (HttpURLConnection) (new URL(url, "library/findByName/" + libraryName).openConnection());
                    connection.setRequestProperty("Accept-Charset", "UTF-8");

                    int responseCode = connection.getResponseCode();

                    if (responseCode != 200) {
                        errors.setText(R.string.error_failed_to_connect);
                        errors.setVisibility(View.VISIBLE);
                        return;

                    }

                    library = Json.read(readIntoString(connection.getInputStream()));

                    connection.disconnect();
                } catch (IOException e) {
                    errors.setText(R.string.error_failed_to_connect);
                    errors.setVisibility(View.VISIBLE);
                    return;
                }

                if (library == null || library.isNull()) {
                    errors.setText(R.string.error_could_not_find_library);
                    errors.setVisibility(View.VISIBLE);
                    return;
                }

                Json device;

                try {
                    HttpURLConnection connection = (HttpURLConnection) (new URL(url, "library/" + library.at("id").asInteger() + "/device").openConnection());
                    connection.setRequestMethod("POST");
                    connection.getResponseCode();
                    device = Json.read(readIntoString(connection.getInputStream()));
                    connection.disconnect();

                    int responseCode = connection.getResponseCode();

                    if (responseCode != 200) {
                        errors.setText(R.string.error_failed_to_initialize_new_device);
                        errors.setVisibility(View.VISIBLE);
                        return;

                    }

                } catch (IOException e) {
                    errors.setText(R.string.error_failed_to_initialize_new_device);
                    errors.setVisibility(View.VISIBLE);
                    return;
                }

                if (device == null || device.isNull()) {
                    errors.setText(R.string.error_failed_to_initialize_new_device);
                    errors.setVisibility(View.VISIBLE);
                    return;
                }

                // This device has been successfully initialized. Save the necessary preferences and restart this activity.
                SharedPreferences.Editor editor = preferences.edit();

                editor.putInt("libraryId", library.at("id").asInteger());
                editor.putInt("deviceId", device.at("id").asInteger());
                editor.putString("hostname", editHostname.getText().toString());
                editor.putBoolean("disableAllInput", disableAllInputCheckbox.isChecked());
                editor.commit();

                Intent intent = getIntent();
                finish();
                startActivity(intent);
            }
        };

        System.out.println(listener == null);

        button.setOnClickListener(listener);
    }

    @Override
    public void onStop() {
        super.onStop();
        stopUpdateTask();
    }

    Handler handler = new Handler();
    private final static int INTERVAL = 5000; // 5 seconds 1000 * 60 * 5; // 5 minutes

    void startUpdateTask() {
        updateTask.run();
    }

    void stopUpdateTask() {
        handler.removeCallbacks(updateTask);
    }

    Runnable updateTask = new Runnable() {
        @Override
        public void run() {
            doUpdateTask();
            handler.postDelayed(updateTask, INTERVAL);
        }
    };

    void doUpdateTask() {
        try {
            SharedPreferences preferences = getPreferences(Context.MODE_PRIVATE);

            int libraryId = preferences.getInt("libraryId", -1);
            int deviceId = preferences.getInt("deviceId", -1);
            String hostname = preferences.getString("hostname", null);
            boolean disableAllInput = preferences.getBoolean("disabledAllInput", false);

            if (libraryId == -1 || deviceId == -1 || hostname == null) {
                throw new AssertionError("Update task started but device has not been properly configured.");
            }

            IntentFilter intentFilter = new IntentFilter(Intent.ACTION_BATTERY_CHANGED);
            Intent batteryStatus = getApplicationContext().registerReceiver(null, intentFilter);

            assert batteryStatus != null;
            int level = batteryStatus.getIntExtra(BatteryManager.EXTRA_LEVEL, -1);
            int scale = batteryStatus.getIntExtra(BatteryManager.EXTRA_SCALE, -1);

            int charge = (int) (level * 100 / (float) scale);

            URL url = new URL("http://" + hostname);

            HttpURLConnection connection = (HttpURLConnection) (new URL(url, "device/" + deviceId).openConnection());
            connection.setRequestProperty("Accept-Charset", "UTF-8");
            int responseCode = connection.getResponseCode();

            if (responseCode == 403) {
                connection.disconnect();

                // This device has become uninitialized.
                SharedPreferences.Editor editor = preferences.edit();
                editor.putInt("libraryId", -1);
                editor.putInt("deviceId", -1);
                editor.putString("hostname", null);
                editor.putBoolean("disableAllInput", false);
                editor.commit();
                stopUpdateTask();

                if (disableAllInput) {
                    // If inputs are disabled, then the only option to re-enabled them is to reboot the device.
                    try {
                        Process process = Runtime.getRuntime().exec(new String[] {"su", "-c", "reboot", "now"});
                        process.waitFor();
                    } catch (IOException | InterruptedException e) {
                        e.printStackTrace();
                    }
                } else {
                    // Otherwise we can immediately restart this activity.
                    Intent intent = getIntent();
                    finish();
                    startActivity(intent);
                    return;
                }
            }

            String input = readIntoString(connection.getInputStream());

            if (input.length() == 0) {
                // Failed to read input. Try again.
                return;
            }

            Json device = Json.read(input);
            connection.disconnect();

            if (!device.at("image_hash").isNull() && !device.at("image_hash").asString().equals(imageHash)) {
                System.out.println("Updating image.");

                imageHash = device.at("image_hash").asString();

                ImageView imageView = (ImageView) findViewById(R.id.imageView);

                connection = (HttpURLConnection) (new URL(url, "device/" + deviceId + "/image").openConnection());
                Bitmap bitmap = BitmapFactory.decodeStream(connection.getInputStream());
                connection.disconnect();
                imageView.setImageBitmap(bitmap);
            }

            if (!device.at("charge").isNull() && device.at("charge").asInteger() != charge) {
                System.out.println("Updating charge.");

                connection = (HttpURLConnection) (new URL(url, "device/" + deviceId + "/charge?charge=" + charge).openConnection());
                connection.setRequestMethod("POST");
                connection.getResponseCode();
                connection.disconnect();
            }
        } catch (IOException e) {
            System.out.println("An exception occurred while running update task.");
            e.printStackTrace();
        }
    }

    /**
     * Disable all input on this device. This operation will silently fail if the current device is
     * not rooted. This operation cannot be reversed and all inputs will be disabled until the
     * current device is restarted.
     */
    static void disableAllInput() {
        // Disable all inputs on this device
        // This is the nuclear option because inputs will be disabled until the device is reset
        try {
            Process process = Runtime.getRuntime().exec("su");
            DataOutputStream out = new DataOutputStream(process.getOutputStream());

            // Disable all inputs on this device
            out.writeBytes("rm /dev/input/event0\n");
            out.writeBytes("rm /dev/input/event1\n");
            out.writeBytes("rm /dev/input/event2\n");
            out.flush();
            out.close();
            process.waitFor();
        } catch (IOException | InterruptedException e) {
            e.printStackTrace();
        }
    }

    static String readIntoString(InputStream stream) {
        java.io.Reader reader = null;
        try {
            reader = new java.io.InputStreamReader(stream);
            StringBuilder content = new StringBuilder();
            char[] buf = new char[1024];
            for (int n = reader.read(buf); n > -1; n = reader.read(buf))
                content.append(buf, 0, n);
            return content.toString();
        } catch (Exception ex) {
            throw new RuntimeException(ex);
        } finally {
            if (reader != null) try {
                reader.close();
            } catch (Throwable ignored) {
            }
        }
    }
}
