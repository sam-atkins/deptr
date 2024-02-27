from fastapi import FastAPI
import pydantic

app = FastAPI()


@app.get("/")
async def root():
    return {"message": "Hello World"}
