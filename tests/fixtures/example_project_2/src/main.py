import json
import os
import re
import sys
from datetime import datetime
from pathlib import Path

from fastapi import FastAPI

app = FastAPI()


@app.get("/")
async def root():
    return {"message": "Hello World"}
