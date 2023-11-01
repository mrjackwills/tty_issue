### Readme
This is a very basic example of the issue I am running into when trying to "suspend" ratatui in order to access a docker container.

1) Required: Rust, and docker, there is a `.devcontainer` directory, which means when VSCode can open the project in a Docker Dev Container.
2) The application runs, and displays button presses
3) 'q' will quit
4) 'e' will enter the Docker Exec mode, this will create an alpine docker container - this may take some time, to download and set up the container.
5) you're now in the docker container, run `ls`, `top` etc to see it all works
6) exit the container, either via `exit` or `ctrl + d`
7) Now you're back in the ratatui application, the initial some key presses will be ignored, others will display the wrong char e.g. pressing down will show `[B`, instead of `Down`
8) It should, after pressing a key once or twice, depending on the key, work as normal now
