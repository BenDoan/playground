import os

from lib.bottle import route, run, default_app

from hue_api import HueAPI

CONFIG_USERNAME = "HUE_API_USERNAME"

def get_api():
    if CONFIG_USERNAME not in os.environ:
        raise Exception("Couldn't find env var {}".format(CONFIG_USERNAME))

    username = os.getenv(CONFIG_USERNAME)
    return HueAPI(username)

api = get_api()

@route('/on')
def all_on():
    api.set_groups(True)
    return "Good"

@route('/off')
def all_off():
    api.set_groups(False)
    return "Good"

if __name__ == '__main__':
    run(host='0.0.0.0', port=9090, debug=True)

app = default_app()
