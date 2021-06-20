package com.github.josephbgerber.nookesign;

import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.os.BatteryManager;
import android.os.Handler;
import android.widget.ImageView;

import com.google.gson.Gson;

import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.MalformedURLException;
import java.net.URL;
import java.net.URLEncoder;
import java.util.Locale;

public class UpdateTask {
    private final static int INTERVAL = 5000; // 5 seconds 1000 * 60 * 5; // 5 minutes

    String image_hash;

    Handler handler = new Handler();
    Context context;
    ImageView imageView;

    UpdateTask(Context context, ImageView imageView) {
        this.context = context;
        this.imageView = imageView;
    }

    Runnable handlerTask = new Runnable()
    {
        @Override
        public void run() {
            doUpdateTask();
            handler.postDelayed(handlerTask, INTERVAL);
        }
    };

    void startRepeatingTask()
    {
        handlerTask.run();
    }

    void stopRepeatingTask()
    {
        handler.removeCallbacks(handlerTask);
    }

    void doUpdateTask() {
        try {
            IntentFilter intentFilter = new IntentFilter(Intent.ACTION_BATTERY_CHANGED);
            Intent batteryStatus = context.registerReceiver(null, intentFilter);

            assert batteryStatus != null;
            int level = batteryStatus.getIntExtra(BatteryManager.EXTRA_LEVEL, -1);
            int scale = batteryStatus.getIntExtra(BatteryManager.EXTRA_SCALE, -1);

            int charge = (int) (level * 100 / (float)scale);

            String charset = "UTF-8";

            // TODO JOE DEVICE SPECIFIC
            int id = 1;

            String endpoint = "http://192.168.1.10:8080/device/" + id;

            URL url = new URL(endpoint);
            HttpURLConnection connection = (HttpURLConnection) url.openConnection();
            connection.setRequestProperty("Accept-Charset", charset);

            Status status = new Gson().fromJson(new InputStreamReader(connection.getInputStream(), "UTF-8"), Status.class);

            System.out.println(status);

            connection.disconnect();

            if (status.image_hash != null && !status.image_hash.equals(image_hash)) {
                System.out.println("Updating image.");

                image_hash = status.image_hash;

                endpoint = "http://192.168.1.10:8080/device/" + id + "/image";


                url = new URL(endpoint);
                connection = (HttpURLConnection) url.openConnection();
                Bitmap bitmap = BitmapFactory.decodeStream(connection.getInputStream());
                connection.disconnect();

                imageView.setImageBitmap(bitmap);
            }

            if (status.charge != charge) {
                System.out.println("Updating charge.");

                endpoint = "http://192.168.1.10:8080/device/" + id + "/charge";
                String query = String.format(
                        Locale.getDefault(),
                        "charge=%s",
                        charge
                );


                url = new URL(endpoint + "?" + query);
                connection = (HttpURLConnection) url.openConnection();
                connection.setRequestMethod("POST");
                connection.getResponseCode();
                connection.disconnect();
            }


        } catch (IOException e) {
            e.printStackTrace();
        }
    }

}
