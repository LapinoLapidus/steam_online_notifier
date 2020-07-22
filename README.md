# Steam Online Notifier
A simple executable that'll give you a system alert when one of your selected Steam friends come online.

### How?
This works by simply querying the Steam Web API every DELAY (specified in .env).

### Installation
Simply clone the git repository or download the [latest release](https://github.com/LapinoLapidus/steam_online_notifier/releases).
Then, rename `.env.example` to `.env` and fill in the required variables.
Delay is in miliseconds, so 10000 would check every 10 seconds. <br/> 
Note that the Steam Web API limits you to 100,000 requests per day. <br/>
You can generate your API Key [here](https://steamcommunity.com/dev/apikey). <br/>
In the `steam_ids.json`, add the STEAMID64's of the people you want to get notifications of as a string array. (See file as example.)<br/>
**The executable must be in the same folder as the `.env` file and the `steam_ids.json` file.*<br/>
**I AM NOT RESPONSIBLE IF YOU EXCEED THE RATELIMITS FOR YOUR STEAM WEB API KEY!**
### Why?
I have a lot of friends on Steam and I only want to know when a select few come online. Steam has an all/no notification setting, so that's why I made this.