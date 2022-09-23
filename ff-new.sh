
# -filter_complex "[0:v]scale=hd1080,setsar=1[v0]; \
#     [1:v]scale=hd1080,setsar=1[v1]; \
#     [2:v]scale=hd1080,setsar=1[v2]; \
#     [3:v]scale=hd1080,setsar=1[v3]; \
#     [v0][0:a][v3][4:a][v1][1:a][v2][2:a]concat=n=4:v=1:a=1" \
ffmpeg \
-hwaccel cuda \
-f lavfi -t 0.1 -i anullsrc=channel_layout=stereo:sample_rate=44100 \
-i 片头.mp4 \
-loop 1 -framerate 25 -t 3 -i Slide1.PNG \
-i input.mp4 \
-i 片尾.mp4 \
-fps_mode vfr \
-filter_complex "[1:v]setsar=1[v1]; \
    [2:v]scale=hd1080,setsar=1[v2]; \
    [3:v]scale=hd1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2,setsar=1[v3]; \
    [4:v]setsar=1[v4]; \
    [v1][1:a][v2][0:a][v3][3:a][v4][4:a]concat=n=4:v=1:a=1" \
out-new.mp4
