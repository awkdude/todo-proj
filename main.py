import torch
from transformers import AutoTokenizer, AutoModelForSequenceClassification
from fastapi import FastAPI
from pydantic import BaseModel
from fastapi.middleware.cors import CORSMiddleware

# 1. Define your local path
local_path = "./final_task_model2"

# 2. Load the tokenizer and model from your local folder
# AutoModelForSequenceClassification is standard for "classifiers"
tokenizer = AutoTokenizer.from_pretrained(local_path)
model = AutoModelForSequenceClassification.from_pretrained(local_path)
model.eval() # Maybe delete

print('model running')

app = FastAPI()
origins = [
    "http://localhost:7878",  # Common for React/Frontend dev
    "http://127.0.0.1:7878",
    # "http://your-production-app.com", 
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,            # Or ["*"] for public access
    allow_credentials=True,
    allow_methods=["*"],              # Allows GET, POST, etc.
    allow_headers=["*"],              # Allows all headers
)

class TaskRequest(BaseModel):
    title: str

# 3. Prepare your input text
@app.post("/predict")
def get_prediction(request: TaskRequest):
    print(f'title: {request.title}')

# while True:
#     text = input('>')
#     if len(text) < 2:
#         break
    

# 4. Tokenize specifically for DistilBERT
# We set return_token_type_ids=False to avoid that TypeError
    inputs = tokenizer(
        request.title, 
        return_tensors="pt", 
        return_token_type_ids=False
    )

# 5. Run the model (The Classifier)
    model.eval() # Set to evaluation mode
    with torch.no_grad():
        outputs = model(**inputs)

# 6. Get the results
    logits = outputs.logits
    predictions = torch.argmax(logits, dim=-1)
    label = model.config.id2label[predictions.item()]
    return {"category_index": predictions.item(), "category_name": label}

    # print(f"Logits: {logits}")
    # print(f"Predicted Class ID: {predictions.item()}")
    # print(f"Label: {label}")
