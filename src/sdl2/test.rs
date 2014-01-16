///  build : rustpkg test sdl2

extern mod sdl2;

//use sdl2::{init, InitEverything, quit};

#[allow(type_overflow)]
#[cfg(test)]
mod tests {
    use sdl2::video;
    #[test]
    fn test_video() {

        let nbdrv = video::SDL_GetNumVideoDrivers();
        println!("num video driver {:?}", nbdrv);

        let drvname = video::SDL_GetVideoDriver(0);
        println!("name of video driver {:?}", drvname);

        let drv = video::SDL_GetVideoDriver(0);
        println!("video driver {:?}", drv);

        let init = video::SDL_VideoInit(drv);
        assert_eq!(init, true);

        let drv2 = video::SDL_GetCurrentVideoDriver();
        println!("video driver {:?}", drv2);
        assert_eq!(drv, drv2);

        let nbvid = video::SDL_GetNumVideoDisplays();
        println!("nb display {:?}", nbvid);

        let dipname = video::SDL_GetDisplayName(0);
        println!("display name {:?}", dipname);

        match (video::SDL_GetDisplayBounds(0))   {
            Some(rect) => println!("display rect {:?}", rect),
            None => fail!("bad display bound")
        }

        let nbmode = video::SDL_GetNumDisplayModes(0);
        println!("nb mode {:?}", nbmode);

        match (video::SDL_GetDisplayMode(0, 0))   {
            Some(mode) => println!("SDL_GetDisplayMode 0,0 {:?}", mode),
            None => fail!("fail SDL_GetDisplayMode")
        }

        if (!super::sdl2::init(&[super::sdl2::InitEverything])){
            fail!("Error, cannot init SDL2")
        }


        let win = video::Window::create_window("Hello World!", 100, 100, 640, 480, &[video::SDL_WINDOW_SHOWN]);

        let flags = win.get_window_flags();

        println!("windows flags {:?}", flags);

        win.set_window_title(&"test title");
        let title = win.get_window_title();
        assert_eq!(title, ~"test title");

        win.set_window_position(50, 60);
        let (x,y) = win.get_window_position();
        assert_eq!(x, 50);
        assert_eq!(y, 60);

        win.set_window_size(500, 510);
        let (x,y) = win.get_window_size();
        assert_eq!(x, 500);
        assert_eq!(y, 510);

        win.set_window_minimum_size(200, 210);
        let (x,y) = win.get_window_minimum_size();
        assert_eq!(x, 200);
        assert_eq!(y, 210);

        win.set_window_maximum_size(600, 610);
        let (x,y) = win.get_window_maximum_size();
        assert_eq!(x, 600);
        assert_eq!(y, 610);

        super::sdl2::quit();

    }

}
