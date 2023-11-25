# RustyChasers

Genetic learning algorithm written in Rust with the macroquad graphics library.
Extended fork of [Shorelark](https://github.com/Patryk27/shorelark) written bottom-up with [Learning to Fly](https://pwy.io/posts/learning-to-fly-pt1/). <br>

Added an adverserial system of chasers (carnivores) and herbivores (as opposed to only "herbivores"). The carnivores' goal is to eat the herbivores. The herbivores goal is to not die while eating as many plants as possible. Code and learning architecture rewritten and expanded to accomodate this. Available models are POSITIONAL, CELLULAR and CLOSEST. Settings can be modified/fine-tuned in /app/src/main.rs. Project can be compiled with ```cargo build``` or ```cargo run```. Add the --release tag for acceptable performance: ```cargo run --release```. <br> <br>

Window example: <br>
<img src="https://github.com/hiddenMedic/rustyChasers/blob/main/images/example.png?raw=true">
Adjustable Settings: <br>
<img src="https://github.com/hiddenMedic/rustyChasers/blob/main/images/settings.png?raw=true">

<br> /py_gen_plotter is used for plotting fitness. Examples: <br>
pos_vis_safe_evolve: <br>
<img src="https://github.com/hiddenMedic/rustyChasers/blob/main/_data_archive/instances/pos_vis_safe_evolve/images/chaser_avg.png?raw=true">
<img src="https://github.com/hiddenMedic/rustyChasers/blob/main/_data_archive/instances/pos_vis_safe_evolve/images/hervor_avg.png?raw=true">

closest_safe_evolve: <br>
<img src="https://github.com/hiddenMedic/rustyChasers/blob/main/_data_archive/instances/closest_safe_evolve/images/chaser_avg.png?raw=true">
<img src="https://github.com/hiddenMedic/rustyChasers/blob/main/_data_archive/instances/closest_safe_evolve/images/hervor_avg.png?raw=true">

Refer to \_data\_archive/instances for more figures.
