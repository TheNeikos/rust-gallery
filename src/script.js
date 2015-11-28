var cur_file = 0;

function set_next_file(i) {
    cur_file = (cur_file + i) % files.length;
    var act_file = files[cur_file];
    img = document.getElementById('image');
    img.src = act_file.path;
}

set_next_file(0);

function keypress(event) {
    var key = event.keyCode || event.which;
    var keychar = String.fromCharCode(key);
    console.log("Whaat ", keychar);

    if (keychar == "H") {
        set_next_file(-1);
    }

    if (keychar == "L") {
        set_next_file(1);
    }
};
