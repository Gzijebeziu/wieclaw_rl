#[derive(PartialEq, Copy, Clone)]
pub struct PrefabLevel {
    pub template : &'static str,
    pub width : usize,
    pub height : usize
}

pub const WFC_POPULATED : PrefabLevel = PrefabLevel{
    template : LEVEL_MAP,
    width: 80,
    height: 43
};

const LEVEL_MAP : &str =
"
################################################################################
#          ########################################################    #########
#    ☺     ######    #########       ####     ###################        #######
#          ####      #                          ###############            #####
#          #### # g  # #######       ####       #############                ###
##### ######### #    # #######       #########  ####    #####                ###
##### ######### ###### #######   G   #########  #### ## #####                ###
##                        ####       #########   ### ##           G     G    ###
##### ######### ###       ####       #######         ## #####                ###
##### ######### ###       ####       ####### #   ### ## #####                ###
##### ######### ###       ####       ####### #######    #####                ###
###          ## ###       ####       ####### ################                ###
###          ## ###   G   ###### ########### #   ############                ###
###          ## ###       ###### ###########     ###                         ###
###    %       %          ###### ########### #   ###   ∞   ##                ###
###          ## ###              ######   ## #######       ##                ###
###          ## ###       ## ### #####     # ########################      #####
###          ## ###       ## ### #####     # #   ######################    #####
#### ## ####### ###### ##### ### ####          G ###########     ######    #####
#### ## ####### ###### ####   ## ####        #   #########         ###### ######
#    ## ####### ###### ####   ## ####        ############           ##### ######
# g  ## ####### ###### ####   ##        %    ###########   G      G  #### #    #
#    ## ###            ####   ## ####        #   #######   ##    ##  ####   g  #
#######                  ####### ####            ######     ∞    ∞    ### #    #
######                     ##### ####        #   ######               ### ######
#####           ∞                #####     # ##########               ### ######
#####                       ### ######     # ##########      G##G     ### #   ##
#####                       ### #######   ## #   ######               ###   g ##
#   ##                     #### ######## ###   G #######  ^########^ #### #   ##
# g    #                 ###### ######## #####   #######  ^        ^ #### ######
#   ##g####           ######    ######## ################           ##### ######
#   ## ########## ##########    ######## #################         ######      #
#####   ######### ########## %  ######## ###################     ######## ##   #
#### ### ######## ##########    ######## #################### ##########   #   #
### ##### ######   #########    ########          ########### #######   # g#   #
### #####           ###############      ###      ########### #######   #### ^ #
### ##### ####       ############## ######## g  g ########### ####         #   #
#### ###^####         ############# ########      #####       ####      # g#   #
#####   ######       ###            ########      ##### g     ####   ∞  ####^^ #
#∞ ^## ###  ##           ########## ########  gg                 g         # > #
#∞%^   ###  ###     ############### ########      ##### g     ####      # g#   #
#∞%^##   ^  ###     ############### ########      #####       ##################
################################################################################
";