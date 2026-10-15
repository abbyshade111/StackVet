import os
from flask import Flask, request

app = Flask(__name__)


@app.route("/control")
def control():
    os.system("ls " + request.args["a"])
    return "ok"


@app.route("/lgtm-same-line")
def lgtm_same_line():
    os.system("ls " + request.args["b"])  # lgtm[py/command-line-injection]
    return "ok"


@app.route("/codeql-line-before")
def codeql_line_before():
    # codeql[py/command-line-injection]
    os.system("ls " + request.args["c"])
    return "ok"


@app.route("/codeql-same-line")
def codeql_same_line():
    os.system("ls " + request.args["d"])  # codeql[py/command-line-injection]
    return "ok"
