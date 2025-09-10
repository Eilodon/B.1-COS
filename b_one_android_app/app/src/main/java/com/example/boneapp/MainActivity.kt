package com.example.boneapp

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import com.b_one.android.engine.BOneEngine
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                Surface {
                    Greeting()
                }
            }
        }
    }
}

@Composable
fun Greeting() {
    val coreVersion = try {
        BOneEngine.getCoreVersion()
    } catch (e: UnsatisfiedLinkError) {
        "Native lib not loaded"
    }
    Text(text = "Status: $coreVersion")
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    Greeting()
}
