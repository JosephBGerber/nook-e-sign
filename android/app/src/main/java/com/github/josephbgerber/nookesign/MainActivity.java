package com.github.josephbgerber.nookesign;

import android.app.Activity;
import android.content.res.Resources;
import android.os.Bundle;
import android.view.KeyEvent;
import android.widget.ImageView;

import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;


public class MainActivity extends Activity {

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

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


        Resources res = getResources();

        String message;

        ImageView imageView = (ImageView) findViewById(R.id.imageView);


        UpdateTask updateTask = new UpdateTask(getApplicationContext(), imageView);
        updateTask.startRepeatingTask();


//    TextView t = (TextView) findViewById(R.id.title);
//    t.setText( message );
    }

//  public void doModConfig(View view) {
//    Intent intent = new Intent(this, ModPrefs.class);
//    startActivity(intent);
//  }
//
//  public void doButtonConfig(View view) {
//    Intent intent = new Intent(this, ButtonPrefs.class);
//    startActivity(intent);
//  }

    @Override
    public boolean onKeyDown(int keyCode, KeyEvent event) {
        // Disable all hardware buttons
        // This will prevent users from exiting the application once it has entered e-screen mode

        return true;
    }
}
