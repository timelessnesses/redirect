import flask
from flask import request
from json import dump,load
from random import sample
from string import ascii_letters,digits
from datetime import timedelta
from time import time
app = flask.Flask(__name__)
class write:
	pass
class read:
	pass
def writeurl(ranstr:str,url:str):
	with open("redirect.json","r") as r:
		redirects = load(r)
		redirects[ranstr] = url
		r.close()
	with open("redirect.json","w") as r:
		dump(redirects,r)
		r.close()
		return True
def readurl(ranstr:str):
	with open("redirect.json") as r:
		redirects = load(r)
		if ranstr not in list(redirects):
			return flask.render_template_string("<h1>No redirect in database found!</h1> Check your Redirect URL! It is CASE SENSITIVE!")
		r.close()
		return redirects[ranstr]
def randomize():
	return ''.join(sample(ascii_letters+digits,10))
	
@app.route('/<redirects>')
def redirect(redirects):
	urls = readurl(redirects)
	if type(urls) is flask.render_template_string:
		return urls
	return ".",302,{"Refresh":f"0; {urls}"}
@app.route('/add')
def add():
	url = flask.request.args.get("url")
	randomized =  randomize()
	try:
		writeurl(randomized,url)
		return "https://redirect.biomooping.tk/"+randomized
	except:
		return "Error!"
@app.errorhandler(404)
def server(*agrs):
	with open("reports") as i:
		yes = load(i)
		yes[str(timedelta(seconds=time()))] = "Server overload or error"
	with open("reports","w") as i:
		dump(yes,i)
		return "Not existent what do you expect?",404
app.run("0.0.0.0",80,debug=True)