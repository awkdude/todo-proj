from transformers import AutoTokenizer, AutoModelForSequenceClassification, Trainer, TrainingArguments
from datasets import load_dataset

# 1. Load Dataset and Labels
dataset = load_dataset('csv', data_files='labeled_tasks.csv')
labels = ["Work", "School", "Personal", "Errands", "Health", "Social"]
id2label = {i: label for i, label in enumerate(labels)}
label2id = {label: i for i, label in enumerate(labels)}

# 2. Tokenize
tokenizer = AutoTokenizer.from_pretrained("distilbert-base-uncased")

def preprocess_function(examples):
    # Convert text to tokens and labels to integers
    tokenized = tokenizer(examples["text"], truncation=True, padding="max_length")
    tokenized["label"] = [label2id[l] for l in examples["label"]]
    return tokenized

tokenized_tasks = dataset.map(preprocess_function, batched=True)

# 3. Load Model with Classification Head
model = AutoModelForSequenceClassification.from_pretrained(
    "distilbert-base-uncased", 
    num_labels=len(labels),
    id2label=id2label,
    label2id=label2id
)

# 4. Train
training_args = TrainingArguments(
    output_dir="./task_classifier",
    learning_rate=2e-5,
    per_device_train_batch_size=16,
    num_train_epochs=5,
    weight_decay=0.01,
)

trainer = Trainer(
    model=model,
    args=training_args,
    train_dataset=tokenized_tasks["train"],
    tokenizer=tokenizer,
)

trainer.train()
model.save_pretrained("./final_task_model")
tokenizer.save_pretrained('./final_task_model')
