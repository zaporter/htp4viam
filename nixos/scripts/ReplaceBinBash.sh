#!/usr/bin/env bash

grep -rl "#!/bin/bash" . | xargs sed -i 's/#!\/bin\/bash/#!\/usr\/bin\/env bash/g'
