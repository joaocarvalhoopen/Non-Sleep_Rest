# Non-Sleep Rest
Two techniques that I believe that are backed by science.

## Description

Prof. Andrew Huberman is at the head of a research lab in Stanford in the area of neuroscience. He has a prolific video podcast, **Huberman Lab** [https://www.youtube.com/c/AndrewHubermanLab](https://www.youtube.com/c/AndrewHubermanLab), where he tries to educate people on the latest advances in the scientific knowledge on neuroscience from papers, that can be used in the daily life. <br>
<br>

While performing heavy intellectual work or studying, it's important that the mind is well rested. <br>
<br>

In this context he talked about two techniques that enable a person to rest in the case that a person hasn't sleep properly or are not well rested and needs to perform at it's best. <br>


## Non-Sleep Deep Rest

This is a technique that uses a kind of light hypnosis to provide a Non-Sleep Deep Rest. This specific youtube video with the guided procedure is recommended by Prof. Andrew Huberman in the following video. <br>

* **ReThink Education - The Biology of Learning Featuring Dr. Andrew Huberman** <br>
  [https://www.youtube.com/watch?v=Oo7hQapFe3M](https://www.youtube.com/watch?v=Oo7hQapFe3M)

* **Non-Sleep Deep Rest - NSDR - A Science Supported Tool for De-Stress and Relaxation - Madefor** <br>
  [https://www.youtube.com/watch?v=pL02HRFk2vo](https://www.youtube.com/watch?v=pL02HRFk2vo)


## 40 Hz binaural beats frequency

Play with ear phones a 40 Hz binaural beat between the left and the right ear. <br>
Left channel 420 Hz <br>
Right channel 460 Hz <br>
Beat 40 Hz <br>
Duration 30 minutes <br>
Amplitude 0.2 <br>
<br>

The sin(x) function is better then the cos(x) because it will start naturally at zero amplitude (so it doesn't make crack noises on the ear phones speakers at the beginning and end of the WAV), but other then that, the two are the same, only shifted ```Pi/2``` radians or 90 degree. <br>
In my first approach to this I used the free program Audacity to generate the WAV audio file maybe after that I will make a program to generate the audio, this github will be a repository for that. <br>
<br>

**Steps to generate the WAV file in Audacity:** <br>

1. Generate 1º track or track mono left in the menu.
2. Generate a second track or track mono right in the menu.
 
3. Select the 1º track and in the menu generate, do generate a tone 420 HZ (left) amplitude 0.2, duration 30 minutes.
4. In the left panel, on the pan slide that has LEFT and RIGHT, pull the signal all to the LEFT.
   
5. Select the 2º track and in the menu generate, do generate a tone 460 HZ (right) amplitude 0.2, duration 30 minutes.
6. In the left panel, on the pan slide that has LEFT and RIGHT, pull the signal all to the RIGHT.
 
7. Select the drop down menu in the audio track on the left panel, it has an option to create a new track, a stereo track from two mono tracks. The first one in the left channel and the second one in the right channel.
 
8. Then export the file to WAV, by doing menu->File->Export->Export WAV and save in some place or path, It's 300 Mega Bytes for 30 minutes. Don't use compressed files, because in compression what is not audible or perceptive in the mp3 will be removed and it's not what we want to do in this case. If you have Linux and only want to test this out save it on the ```/dev/shm/``` . It's a RAM disk that all Linux systems have by default.

Then make play by double clicking on the WAV file and adjust the audio volume so that you can hear it, but it's a low volume, it works even at low volumes . It's not a irritating sound. <br>
<br>

I have found this frequencies and parameters in a paper that was in the middle of the description of the Huberman Lab video podcast. <br>
When I tested this some months ago, and in that day I was particularly tired because I had been burning in the previous night the midnight oil while finishing a small project for my github. <br>


* **Paper - 40-Hz Binaural beats enhance training to mitigate the attentional blink** <br>
  [https://www.nature.com/articles/s41598-020-63980-y](https://www.nature.com/articles/s41598-020-63980-y)

* **Optimizing Workspace for Productivity, Focus, Creativity - Huberman Lab** <br>
  [https://www.youtube.com/watch?v=Ze2pc6NwsHQ](https://www.youtube.com/watch?v=Ze2pc6NwsHQ)

* In the last video, **the exact moment** after which, **Andrew Huberman speaks about this subject.** <br>
  [https://youtu.be/Ze2pc6NwsHQ?t=3343](https://youtu.be/Ze2pc6NwsHQ?t=3343)


### Alternatively I made a small program to generate the 40 Hz Binaural Beat WAV file.

To compile it first you have to download and go to the sub-directory ```binaural_beats_40Hz```. <br>

```
# To compile the final program inside cargo
cargo build --release
 
# To execute the program directly do
Usage: "binaural_beats_40hz wav_target_filename.wav"

 ex: ./binaural_beats_40hz /dev/shm/binaural_beats_40hz.wav
 
# To execute the program inside cargo and save the
# WAV on the RAM disk in Linux. 
 cargo run --release /dev/shm/binaural_beats_40hz.wav
 
# Then double click on the WAV file at /dev/shm/  

# To generate the docs inside cargo
cargo doc
cargo doc --open
```


## References

* **Huberman Lab** <br>
  [https://www.youtube.com/c/AndrewHubermanLab](https://www.youtube.com/c/AndrewHubermanLab)


## Disclaimer

I'm not a neuroscientist, I'm not even a scientist. I'm just a simple person that experimented those two techniques and to myself they appeared to work. But regard, that the placebo effect can always creep on any person. But I convinced myself sufficiently to experiment with those two techniques, by following what was said in Huberman Lab, by Prof. Andrew Huberman. I trust his knowledgeable advice. If you do this stuff do it at your own risk. I'm not giving advice to anyone about this, I am just documenting it for my personal future memory.

## License
The code license for the small program that generates the 40Hz Binaural Beats is MIT Open Source License. 


## Have fun
Best regards, <br>
João Nuno Carvalho
