package com.qktz.vad_rs;

import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.ActivityCompat;
import androidx.core.content.ContextCompat;

import android.Manifest;
import android.content.pm.PackageManager;
import android.media.AudioFormat;
import android.media.AudioRecord;
import android.media.MediaRecorder;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;

import com.qktz.kimivad.KimiVad;

public class MainActivity extends AppCompatActivity {
    private static final int PERMISSION_REQUEST_CODE = 1;
    private static final int SAMPLE_RATE = 44100;
    private static final int CHANNEL_CONFIG = AudioFormat.CHANNEL_IN_MONO;
    private static final int AUDIO_FORMAT = AudioFormat.ENCODING_PCM_16BIT;
    private static final int BUFFER_SIZE = AudioRecord.getMinBufferSize(SAMPLE_RATE, CHANNEL_CONFIG, AUDIO_FORMAT);

    private AudioRecord audioRecord;
    private boolean isRecording = false;
    private Thread recordingThread;
    private Button toggleButton;
    private TextView statusText;
    private long vadHandle;
    private final AudioResampler resampler = new AudioResampler();

    private static final String TAG = "MainActivity";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        toggleButton = findViewById(R.id.toggleButton);
        statusText = findViewById(R.id.statusText);

        toggleButton.setOnClickListener(v -> toggleRecording());

        // 初始化VAD
        vadHandle = KimiVad.init_vad_iter("");

        requestPermissions();
    }

    private void requestPermissions() {
        if (ContextCompat.checkSelfPermission(this, Manifest.permission.RECORD_AUDIO)
                != PackageManager.PERMISSION_GRANTED) {
            ActivityCompat.requestPermissions(this,
                    new String[]{Manifest.permission.RECORD_AUDIO},
                    PERMISSION_REQUEST_CODE);
        }
    }

    private void toggleRecording() {
        if (isRecording) {
            stopRecording();
            toggleButton.setText("开始录音");
        } else {
            startRecording();
            toggleButton.setText("停止录音");
        }
    }

    private void startRecording() {
        if (ActivityCompat.checkSelfPermission(this, Manifest.permission.RECORD_AUDIO) != PackageManager.PERMISSION_GRANTED) {
            // TODO: Consider calling
            //    ActivityCompat#requestPermissions
            // here to request the missing permissions, and then overriding
            //   public void onRequestPermissionsResult(int requestCode, String[] permissions,
            //                                          int[] grantResults)
            // to handle the case where the user grants the permission. See the documentation
            // for ActivityCompat#requestPermissions for more details.
            return;
        }
        audioRecord = new AudioRecord(MediaRecorder.AudioSource.MIC,
                SAMPLE_RATE, CHANNEL_CONFIG, AUDIO_FORMAT, BUFFER_SIZE);

        isRecording = true;
        audioRecord.startRecording();

        recordingThread = new Thread(this::processAudio);
        recordingThread.start();
    }

    private void stopRecording() {
        isRecording = false;
        if (audioRecord != null) {
            audioRecord.stop();
            audioRecord.release();
            audioRecord = null;
        }
    }

    private void processAudio() {
        short[] audioBuffer = new short[BUFFER_SIZE/2];
        
        while (isRecording) {
            int readSize = audioRecord.read(audioBuffer, 0, audioBuffer.length);
            if (readSize > 0) {
                // 重采样到16kHz
                short[] resampledData = resampler.resample(audioBuffer, readSize, SAMPLE_RATE, 16000);
                

                int chunkSize = 512;
                for (int i = 0; i + chunkSize <= resampledData.length; i += chunkSize) {
                    short[] chunk = new short[chunkSize];
                    System.arraycopy(resampledData, i, chunk, 0, chunkSize);

                    byte[] bytes = new byte[chunk.length * 2];
                    for (int j = 0; j < chunk.length; j++) {
                        bytes[j * 2] = (byte) (chunk[j] & 0xff);
                        bytes[j * 2 + 1] = (byte) ((chunk[j] >> 8) & 0xff);
                    }

                    long startTime = System.currentTimeMillis();
                    long result = KimiVad.process_vad_iter(vadHandle, bytes);
                    long endTime = System.currentTimeMillis();
                    Log.i(TAG, "processAudio: " + (endTime - startTime) + "ms");
                    updateUI(result == 1 || result == 2);
                }
            }
        }
    }

    private void updateUI(boolean isSpeaking) {
        runOnUiThread(() -> {
            statusText.setText(isSpeaking ? "正在说话" : "静音中");
            statusText.setBackgroundColor(isSpeaking ? 
                    getResources().getColor(android.R.color.holo_green_light) : 
                    getResources().getColor(android.R.color.holo_red_light));
        });
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        stopRecording();
    }
}