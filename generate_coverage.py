from os import listdir
from os import path
from os import system
import re

project_name = "keygraph_rs"
tests = [path.splitext(f)[0] for f in listdir("tests")]
tests.append(project_name)

pattern = '(' + '|'.join(tests) + ')-*'
regex = re.compile(pattern)

binaries = ['target/debug/'+f for f in listdir("target/debug/") if regex.match(f) and path.splitext(f)[1]!='.d']

targets = []
for i, binary in enumerate(binaries):
    system('kcov --exclude-pattern=/.cargo,/usr/lib,tests --verify target/cov' + str(i) +' '+binary)
    targets.append('target/cov' + str(i))

system('kcov --coveralls-id=$repo_token --merge target/cov ' + ' '.join(targets))
