// All Art is currently used from https://github.com/rwos/gti/blob/master/gti.c

/*
 * gti - a git launcher
 *
 * Copyright 2012 Richard Wossal <richard@r-wos.org> and contributors.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation.  No representations are made about the
 * suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 */

pub const CAR_VARIANT1: &[&str] = &[
    "   ,---------------.",
    "  /  /``````|``````\\\\",
    " /  /_______|_______\\\\________",
    "|]      GTI |'       |        |]",
    "=  .-:-.    |________|  .-:-.  =",
    " `  -+-  --------------  -+-  '",
    "   '-:-'                '-:-'  ",
];

pub const CAR_VARIANT2: &[&str] = &[
    "   ,---------------.",
    "  /  /``````|``````\\\\",
    " /  /_______|_______\\\\________",
    "|]      GTI |'       |        |]",
    "=  .:-:.    |________|  .:-:.  =",
    " `   X   --------------   X   '",
    "   ':-:'                ':-:'  ",
];

pub const CAR_PUSHING1 : &[&str] = &[
    "   __      ,---------------.",
    "  /--\\   /  /``````|``````\\\\",
    "  \\__/  /  /_______|_______\\\\________",
    "   ||-< |]      GTI |'       |        |]",
    "   ||-< =  .-:-.    |________|  .-:-.  =",
    "   ||    `  -+-  --------------  -+-  '",
    "   ||      '-:-'                '-:-'  ",
];

pub const CAR_PUSHING2 : &[&str] = &[
    "   __      ,---------------.",
    "  /--\\   /  /``````|``````\\\\",
    "  \\__/  /  /_______|_______\\\\________",
    "   ||-< |]      GTI |'       |        |]",
    "   ||-< =  .:-:.    |________|  .:-:.  =",
    "   /\\    `   X   --------------   X   '",
    "  /  \\     ':-:'                ':-:'  ",
];

pub const CAR_PULLING1 : &[&str] = &[
    "                                              ------.",
    "   ,---------------.                          |      |       ,",
    "  /  /``````|``````\\\\                         |      |       ||",
    " /  /_______|_______\\\\________            ,-------.--+-------++--,",
    "|]      GTI |'       |        |]           / .:-:.     |          |",
    "=  .-:-.    |________|  .-:-. = -----------     .   `-------  .-:-.",
    " `  -+-  --------------  -+-  '               '       `----'    +",
    "   '-:-'                '-:-'                '-:-'            '-:-'",
];
    

pub const CAR_PULLING2 : &[&str] = &[
    "                                              ------.",
    "   ,---------------.                          |      |       ,",
    "  /  /``````|``````\\\\                         |      |       ||",
    " /  /_______|_______\\\\________            ,-------.--+-------++--,",
    "|]      GTI |'       |        |]           / .-:-.     |          |",
    "=  .:-:.    |________|  .:-:. = -----------   ,     `-------  .:-:.",
    " `   X   --------------   X   '                 '     `----'    X",
    "   ':-:'                ':-:'                ':-:'            ':-:'",
];

pub const FORCE_HAND : &[&str] = &[
    "                     -#%+     ",
    "                    *%+       ",
    "                  =+%    =+*+.",
    "                :#@%%*##%%+   ",
    "              -##@%%@%#       ",
    "        -=:+-=*%#%%@#@%%##    ",
    "   .*=+#%##%%#%###@#*@@+.#%#  ",
    "#**#*###%#%%@%#+**#+  #%%     ",
    "=*%##%%%%%%*=+           #=   ",
    "%#%#%*#%%%#-                  ",
];    
