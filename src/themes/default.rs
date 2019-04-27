use themes::*;

pub fn default_theme() -> Theme {

    let mut theme: Theme = [0; THEME_SIZE];
       
    theme[RESET] = -1;

    theme[ USERNAME_FG] = 250;
    theme[ USERNAME_BG] = 240;
    theme[ USERNAME_ROOT_BG] = 124;

    theme[ HOSTNAME_FG] = 250;
    theme[ HOSTNAME_BG] = 238;

    theme[ HOME_SPECIAL_DISPLAY] = 1;
    theme[ HOME_BG] = 31;  // blueish
    theme[ HOME_FG] = 15;  // white
    theme[ PATH_BG] = 237;  // dark grey
    theme[PATH_FG] = 250;  // light grey
    theme[CWD_FG] = 254;  // nearly-white grey
    theme[SEPARATOR_FG] = 244;

    theme[ READONLY_BG] = 124;
    theme[ READONLY_FG] = 254;

    theme[ SSH_BG] = 166;  // medium orange
    theme[ SSH_FG] = 254;

    theme[ REPO_CLEAN_BG] = 148;  // a light green color
    theme[ REPO_CLEAN_FG] = 0;  // black
    theme[ REPO_DIRTY_BG] = 161;  // pink/red
    theme[ REPO_DIRTY_FG] = 15;  // white

    theme[ JOBS_FG] = 39;
    theme[ JOBS_BG] = 238;

    theme[ CMD_PASSED_BG] = 236;
    theme[ CMD_PASSED_FG] = 15;
    theme[ CMD_FAILED_BG] = 161;
    theme[ CMD_FAILED_FG] = 15;

    theme[ SVN_CHANGES_BG] = 148;
    theme[ SVN_CHANGES_FG] = 22;  // dark green

    theme[ GIT_AHEAD_BG] = 240;
    theme[ GIT_AHEAD_FG] = 250;
    theme[ GIT_BEHIND_BG] = 240;
    theme[ GIT_BEHIND_FG] = 250;
    theme[ GIT_STAGED_BG] = 22;
    theme[ GIT_STAGED_FG] = 15;
    theme[ GIT_NOTSTAGED_BG] = 130;
    theme[ GIT_NOTSTAGED_FG] = 15;
    theme[ GIT_UNTRACKED_BG] = 52;
    theme[ GIT_UNTRACKED_FG] = 15;
    theme[ GIT_CONFLICTED_BG] = 9;
    theme[ GIT_CONFLICTED_FG] = 15;

    theme[ GIT_STASH_BG] = 221;
    theme[ GIT_STASH_FG] = 0;

    theme[ VIRTUAL_ENV_BG] = 35;  // a mid-tone green
    theme[ VIRTUAL_ENV_FG] = 00;

    theme[ BATTERY_NORMAL_BG] = 22;
    theme[ BATTERY_NORMAL_FG] = 7;
    theme[ BATTERY_LOW_BG] = 196;
    theme[ BATTERY_LOW_FG] = 7;

    theme[ AWS_PROFILE_FG] = 39;
    theme[ AWS_PROFILE_BG] = 238;

    theme[ TIME_FG] = 250;
    theme[ TIME_BG] = 238;

    return theme;
}
