#include <vlc/vlc.h>
#include <iostream>
#include <Windows.h>
int main(void)
{
    int argc; const char* argv[] = { "--no-video" };
    libvlc_instance_t * inst;
    libvlc_media_player_t *mp;
    libvlc_media_t *m;
 
    /* Load the VLC engine */
    inst = libvlc_new(1, argv);
 
    /* Create a new item */
    m = libvlc_media_new_location("http://mycool.movie.com/test.mov");
    //m = libvlc_media_new_path("/path/to/test.mov");
 
    /* Create a media player playing environement */
    mp = libvlc_media_player_new_from_media (inst, m);
 
    /* No need to keep the media now */
    libvlc_media_release (m);
 
    /* play the media_player */
    libvlc_media_player_play (mp);
 
    while (libvlc_media_player_is_playing(mp))
    {
        Sleep(1);
        int64_t milliseconds = libvlc_media_player_get_time(mp);
        int64_t seconds = milliseconds / 1000;
        int64_t minutes = seconds / 60;
        milliseconds -= seconds * 1000;
        seconds -= minutes * 60;
 
	std::cout << "Current time: %" << minutes << ":%" << seconds << ":%" << milliseconds << std::endl;
    }
 
    /* Stop playing */
    libvlc_media_player_stop_async (mp);
 
    /* Free the media_player */
    libvlc_media_player_release (mp);
 
    libvlc_release (inst);
 
    return 0;
}
