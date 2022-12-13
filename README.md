# audio_player

# Description 
An audio playback machine providing the bone-structure to a streaming platform for music listening abilities. This program is currently only compatible with wav formatted audio files. 

# Setup
The first step is providing a wav file in the data folder with the following specifications: 16 bit, 44,000khz. Once the file is correctly placed, open a terminal window and navigate to the audio_player folder. Then cd into the wss folder and type cargo run. This initializes the server. Remaining in the audio_player file structure, open a seperate terminal window and run the command wscat -c 127.0.0.1:9001. This opens the connection to an endpoint for the "user". Once connceted, type the name of the wav file, in this case sample_wav, and the wav file will be streamed.

# Audio File Background Information
A wav file is one of the highest quality, commonly found, audio files but larger in size compared to its counterparts. Other popular audio file types are various MP3 qualities (128kps, 250kps, 320kps), aiff, etc... These will be added in the future and are subsantially smaller than wav files. 

# Technologies
Rust, JavaScript, React 
