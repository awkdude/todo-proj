import csv
import random

# Categories and specific keywords to help the model learn patterns
data_map = {
    "Work": ["Work", "Job", "Email", "Boss", "Coworker", "Interview", "Internship", "Meeting", "Report", "Deadline", "Project", "Client", "Update", "Programming", "Meeting", "Design", "Portfolio", "Recruiter", "Manager", "Computer"],
    "School": ["Teacher", "Professor", "Email", "Book", "School", "Exam", "Homework", "Art", "Write", "Read", "Lecture", "Governors State", "Study", "Lab", "Quiz", "Project", "College", "University", "Computer"],
    "Personal": ["Personal", "Decoration", "Dishes", "Kitchen", "Room", "Shave", "Bed", "Hair", "Body", "Clothes", "Shower", "Cook", "Wash", "Dry", "Brush" "Meditate", "Art", "Guitar", "Call Mom", "Call Dad", "Hobby", "Laundry", "Nails", "Vacuum", "Mop", "Beard", "Hand", "Face", "House"],
    "Errands": ["Insurance", "Bill", "Garbage", "Trash", "DMV",  "Food",  "Groceries", "Lunch", "Breakfest", "Milk", "Food" "Gas station", "Pharmacy", "Clean", "Bank", "Mail", "Dinner", "Car", "Plant", "Garden", "Taxes", "Refund", "Service", "Mechanic", "Mall", "Store", "Money"],
    "Health": ["Fruit", "Vegetables", "Health", "Surgery", "Elliptical", "Weight", "Doctor", "Therapist", "Vitamin", "Protein", "Water", "Medicine", "Gym", "Run", "Stretch", "Jog", "Jump rope", "Lift weights", "Walk", "Running", "Lift", "Workout", "Pushups", "Situps"],
    "Social": ["Mom", "Dad", "Sibling", "Friends", "Family", "Club", "Event", "Meet", "Party", "Restaurant", "Food", "Game", "Call", "Birthday", "Movie", "Bar", "Christmas", "Thanksgiving", "New Year's", "4th of July" "Phone", "Gathering", "Catering"],
}

def generate_classification_data(filename="labeled_tasks.csv", num_samples=1500):
    with open(filename, mode='w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(["text", "label"]) # Header

        for _ in range(num_samples):
            category = random.choice(list(data_map.keys()))
            action = random.choice(data_map[category])
            
            # Create a more natural sentence
            templates = [
                f"{action}",
                # f"Need to finish {action}",
                # f"Reminder: {action}",
                # f"Don't forget the {action}",
                f"Finish {action}",
                f"Get {action}",
                f"Do {action}",
                f"Goto {action}",
                f"Go for {action}",
                f"Pay {action}",
                f"Visit {action}",
                f"Schedule {action}",
            ]
            text = random.choice(templates)
            writer.writerow([text, category])

if __name__ == "__main__":
    generate_classification_data()
    print("Dataset 'labeled_tasks.csv' created.")
