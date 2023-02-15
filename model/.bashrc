#!/bin/sh

conda activate directml
echo $LD_LIBRARY_PATH | grep "$CONDA_PREFIX/lib" > /dev/null 2> /dev/null
if [[ $? -ne 0 ]]; then
	export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$CONDA_PREFIX/lib/
fi
echo $PATH | grep "/usr/local/cuda/bin:$PATH" > /dev/null 2> /dev/null
if [[ $? -ne 0 ]]; then
	export PATH=/usr/local/cuda/bin:$PATH
fi
