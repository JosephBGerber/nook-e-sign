package com.github.josephbgerber.nookesign;

import android.app.Activity;
import android.content.res.Resources;
import android.os.Bundle;
import android.view.KeyEvent;
import android.widget.ImageView;


public class MainActivity extends Activity {

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

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
