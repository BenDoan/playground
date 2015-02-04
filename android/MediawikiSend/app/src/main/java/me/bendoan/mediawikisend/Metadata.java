package me.bendoan.mediawikisend;

import android.content.Context;
import android.content.Intent;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.net.Uri;
import android.os.AsyncTask;
import android.os.Environment;
import android.os.StrictMode;
import android.os.SystemClock;
import android.provider.MediaStore;
import android.support.v7.app.ActionBarActivity;
import android.support.v4.app.Fragment;
import android.os.Bundle;
import android.util.Base64;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.Menu;
import android.view.MenuItem;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Toast;

import org.apache.http.conn.ClientConnectionManager;
import org.apache.http.conn.scheme.PlainSocketFactory;
import org.apache.http.conn.scheme.Scheme;
import org.apache.http.conn.scheme.SchemeRegistry;
import org.apache.http.conn.ssl.SSLSocketFactory;
import org.apache.http.impl.client.AbstractHttpClient;
import org.apache.http.impl.client.DefaultHttpClient;
import org.apache.http.impl.conn.tsccm.ThreadSafeClientConnManager;
import org.apache.http.params.BasicHttpParams;
import org.apache.http.params.CoreProtocolPNames;
import org.mediawiki.api.ApiResult;
import org.mediawiki.api.MWApi;
import org.w3c.dom.Node;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.io.StringWriter;
import java.text.SimpleDateFormat;
import java.util.Date;

import javax.xml.transform.TransformerConfigurationException;
import javax.xml.transform.TransformerException;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.TransformerFactoryConfigurationError;


public class Metadata extends ActionBarActivity {

    private static final String TAG = "Mediawiki-Send";
    private static final String API_URL = "http://flainted.com/wiki/api.php";
    private static final String USER_AGENT = "BenDoan MediaWiki Upload App";
    private MWApi api;

    static final int REQUEST_IMAGE_CAPTURE = 1;

    private int upload_percent = 0;
    File photoFile = null;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        //TODO: replace with async
        StrictMode.ThreadPolicy policy = new StrictMode.ThreadPolicy.Builder().permitAll().build();
        StrictMode.setThreadPolicy(policy);

        api = createMWApi();

        Intent i = getIntent();
        Log.i(TAG, "Intent: " + i.getAction());
        if (i.getAction().equals("android.intent.action.SEND")) {
            handleSendIntent(i);
        }else if (i.getAction().equals("android.intent.action.MAIN")){
            dispatchTakePictureIntent();
        }



        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_metadata);
        if (savedInstanceState == null) {
            getSupportFragmentManager().beginTransaction()
                    .add(R.id.container, new PlaceholderFragment())
                    .commit();
        }
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        if (requestCode == REQUEST_IMAGE_CAPTURE && resultCode == RESULT_OK) {
            Log.i(TAG, "Received image");
            new UploadImageTask().execute(Uri.parse(photoFile.toURI().toString()));
        }
    }

/*
    private void dispatchTakePictureIntent() {
        Intent takePictureIntent = new Intent(MediaStore.ACTION_IMAGE_CAPTURE);
        if (takePictureIntent.resolveActivity(getPackageManager()) != null) {
            startActivityForResult(takePictureIntent, REQUEST_IMAGE_CAPTURE);
        }
    }
*/

    private void dispatchTakePictureIntent() {
        Intent takePictureIntent = new Intent(MediaStore.ACTION_IMAGE_CAPTURE);

        // Ensure that there's a camera activity to handle the intent
        if (takePictureIntent.resolveActivity(getPackageManager()) != null) {
            // Create the File where the photo should go
            try {
                photoFile = createImageFile();
            } catch (IOException ex) {
                Log.e(TAG, "Couldn't create file for image");
            }

            // Continue only if the File was successfully created
            if (photoFile != null) {
                takePictureIntent.putExtra(MediaStore.EXTRA_OUTPUT,
                        Uri.fromFile(photoFile));
                startActivityForResult(takePictureIntent, REQUEST_IMAGE_CAPTURE);


            }

        }
    }

    protected void handleSendIntent(Intent intent) {
        String type = intent.getType();
        String action = intent.getAction();

        if (Intent.ACTION_SEND.equals(action) && null != type) {
            if (type.startsWith("image/")) {
                Uri imageUri = intent.getParcelableExtra(Intent.EXTRA_STREAM);
                new UploadImageTask().execute(imageUri);
            }
        } else if (Intent.ACTION_SEND_MULTIPLE.equals(action) && null != type) {
            if (type.startsWith("image/")) {
                //TODO: support multiple images?
            }
        }
    }

    protected void uploadImage(Uri uri) throws IOException {
        String pageContents = "page contents";
        String editSummary = "edit sum";
        String filename = uri.getLastPathSegment();
        InputStream file = getContentResolver().openInputStream(uri);

        String username = "";
        String password = "";

        try {
            Log.i(TAG, "Logging in");
            this.api.login(username, password);
        } catch (final IOException e) {
            Log.e(TAG, "Login failed");
        }

        synchronized (this) {
            if (!this.api.isLoggedIn) {
                Log.e(TAG, "Not logged in");
            }
        }
        final ApiResult result = this.api.upload(filename, file, pageContents, editSummary);

        Log.i(TAG, "Upload result is: " + getStringFromDOM(result.getDocument()));

        final ApiResult error = result.getNode("/api/error");
    }


    private File createImageFile() throws IOException {
        // Create an image file name
        String timeStamp = new SimpleDateFormat("yyyyMMdd_HHmmss").format(new Date());
        String imageFileName = "JPEG_" + timeStamp + "_"; //TODO: choose better name/tmp name
        File storageDir = Environment.getExternalStoragePublicDirectory(
                Environment.DIRECTORY_PICTURES);
        File image = File.createTempFile(
                imageFileName,  /* prefix */
                ".jpg",         /* suffix */
                storageDir      /* directory */
        );

        return image;
    }

    public String getBase64String(Uri uri) {
        Bitmap bm = null;
        InputStream is = null;
        try {
            is = getContentResolver().openInputStream(uri);
            bm = BitmapFactory.decodeStream(is);
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } finally {
            try {
                if (is != null) {
                    is.close();
                }
            } catch (IOException e) {
                e.printStackTrace();
            }
        }

        if (null == bm) {
            return null;
        }
        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        bm.compress(Bitmap.CompressFormat.JPEG, 100, baos); //bm is the bitmap object
        byte[] b = baos.toByteArray();

        return Base64.encodeToString(b, Base64.DEFAULT);
    }

    public void makeToast(String str) {
        Context context = getApplicationContext();
        int duration = Toast.LENGTH_SHORT;

        Toast toast = Toast.makeText(context, str, duration);
        toast.show();
    }
    //TODO: delete
    public static String getStringFromDOM(Node dom) {
        javax.xml.transform.Transformer transformer = null;
        try {
            transformer = TransformerFactory.newInstance().newTransformer();
        } catch (TransformerConfigurationException e) {
            e.printStackTrace();
        } catch (TransformerFactoryConfigurationError e) {
            e.printStackTrace();
        }
        StringWriter outputStream = new StringWriter();
        javax.xml.transform.dom.DOMSource domSource = new javax.xml.transform.dom.DOMSource(dom);
        javax.xml.transform.stream.StreamResult strResult = new javax.xml.transform.stream.StreamResult(outputStream);
        try {
            transformer.transform(domSource, strResult);
        } catch (TransformerException e) {
            e.printStackTrace();
        }
        return outputStream.toString();
    }

    public static MWApi createMWApi() {
        return new MWApi(API_URL, createHttpClient());
    }


    public static AbstractHttpClient createHttpClient() {
        BasicHttpParams params = new BasicHttpParams();
        SchemeRegistry schemeRegistry = new SchemeRegistry();
        schemeRegistry.register(new Scheme("http", PlainSocketFactory.getSocketFactory(), 80));
        final SSLSocketFactory sslSocketFactory = SSLSocketFactory.getSocketFactory();
        schemeRegistry.register(new Scheme("https", sslSocketFactory, 443));
        ClientConnectionManager cm = new ThreadSafeClientConnManager(params, schemeRegistry);
        params.setParameter(CoreProtocolPNames.USER_AGENT, USER_AGENT);
        return new DefaultHttpClient(cm, params);
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        // Inflate the menu; this adds items to the action bar if it is present.
        getMenuInflater().inflate(R.menu.menu_metadata, menu);
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

    /**
     * A placeholder fragment containing a simple view.
     */
    public static class PlaceholderFragment extends Fragment {

        public PlaceholderFragment() {
        }

        @Override
        public View onCreateView(LayoutInflater inflater, ViewGroup container,
                                 Bundle savedInstanceState) {
            return inflater.inflate(R.layout.fragment_metadata, container, false);
        }
    }

    private class UploadImageTask extends AsyncTask<Uri, Integer, Integer> {
        protected Integer doInBackground(Uri... uris) {
            try {
                uploadImage(uris[0]);
            } catch (IOException e) {
                e.printStackTrace();
            }

            publishProgress(100);
            return 100;
        }

        protected void onProgressUpdate(Integer... progress) {
            upload_percent = progress[0];
        }

        protected void onPostExecute(Integer result) {
            Log.i(TAG, "Upload " + result + " bytes");
        }
    }
}




