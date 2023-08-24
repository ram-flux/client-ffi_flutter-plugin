package com.techecho.rfapp;

import android.os.Bundle;

import com.google.android.material.snackbar.Snackbar;

import androidx.appcompat.app.AppCompatActivity;

import android.util.Log;
import android.view.View;

import androidx.navigation.NavController;
import androidx.navigation.Navigation;
import androidx.navigation.ui.AppBarConfiguration;
import androidx.navigation.ui.NavigationUI;

import com.techecho.rfapp.databinding.ActivityMainBinding;

import android.view.Menu;
import android.view.MenuItem;

import java.io.File;

public class MainActivity extends AppCompatActivity {

    private AppBarConfiguration appBarConfiguration;
    private ActivityMainBinding binding;

    // Used to load the 'test_rust_ffi' library on application startup.
    static {
//        System.loadLibrary("test_rust_ffi");
        System.loadLibrary("client_ffi");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        binding = ActivityMainBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());

        setSupportActionBar(binding.toolbar);

        NavController navController = Navigation.findNavController(this, R.id.nav_host_fragment_content_main);
        appBarConfiguration = new AppBarConfiguration.Builder(navController.getGraph()).build();
        NavigationUI.setupActionBarWithNavController(this, navController, appBarConfiguration);

        String req = "{\n" +
                "    \"start_req\": {\n" +
                "        \"type\": \"client\",\n" +
                "        \"set_key_req\": {\n" +
                "            \"prikey\": \"4a70fe852c152df4c23cd01b5ba44bb0a8609b5cb511cef713945b8e25221999\"\n" +
                "        },\n" +
                "        \"assign_interface_req\": {\n" +
                "            \"name\": \"utun\",\n" +
                "            \"num\": 100,\n" +
                "            \"native_external_ip\": \"220.200.5.244\",\n" +
                "            \"ipv4\": \"10.0.1.3\",\n" +
                "            \"ipv6\": \"fd86:ea04:1111::\"\n" +
                "        },\n" +
                "        \"add_transport_req\": {\n" +
                "            \"port\": 5173,\n" +
                "            \"protocol\": \"tcp\",\n" +
                "            \"endpoint\": \"52.221.222.252:5173\"\n" +
                "        },\n" +
                "        \"add_network_segment_req\": {\n" +
                "            \"segments\": [\n" +
                "                \"0.0.0.0/0\"\n" +
                "            ]\n" +
                "        },\n" +
                "        \"add_node_req\": {\n" +
                "            \"pub_key\": \"31d7d3f5ccc1ad0214572918e1de84ec72f93177370cc28df6dd58413425fb58\",\n" +
                "            \"endpoint\": \"52.221.222.252:5173\",\n" +
                "            \"allowed_ips\": [\n" +
                "                \"0.0.0.0/0\"\n" +
                "            ]\n" +
                "        }\n" +
                "    }\n" +
                "}";

        binding.fab.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                // syncCallback
                FFIUtil.syncCallback(
                        new RustListener() {
                            @Override
                            public void onStringCallback(String node, String msg) {
                                Log.d("test", "sync callback: node" + node + ", msg: " + msg);
                            }
                        }
                );
                // test
                FFIUtil.test("aaa");

                // connectToNode
                File internalStorage = getApplicationContext().getFilesDir();
                String path = internalStorage.getAbsolutePath();
                String res = FFIUtil.connectToNode(req, new ConnectStatusListener() {
                    @Override
                    public void onConnectedCallback(String node, String msg) {
                        Log.d("test", "sync onConnectedCallback: node" + node + ", msg: " + msg);
                    }

                    @Override
                    public void onDisconnectedCallback(String node, String msg) {
                        Log.d("test", "sync onDisconnectedCallback: node" + node + ", msg: " + msg);

                    }
                }, path, 5);
                Snackbar.make(view, res, Snackbar.LENGTH_LONG)
                        .setAnchorView(R.id.fab)
                        .setAction("Action", null).show();
            }
        });
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        // Inflate the menu; this adds items to the action bar if it is present.
        getMenuInflater().inflate(R.menu.menu_main, menu);
        return true;
    }

    @Override
    public boolean onOptionsItemSelected(MenuItem item) {
        // Handle action bar item clicks here. The action bar will
        // automatically handle clicks on the Home/Up button, so long
        // as you specify a parent activity in AndroidManifest.xml.
        int id = item.getItemId();

        //noinspection SimplifiableIfStatement
        if (id == R.id.action_settings) {
            return true;
        }

        return super.onOptionsItemSelected(item);
    }

    @Override
    public boolean onSupportNavigateUp() {
        NavController navController = Navigation.findNavController(this, R.id.nav_host_fragment_content_main);
        return NavigationUI.navigateUp(navController, appBarConfiguration)
                || super.onSupportNavigateUp();
    }
}