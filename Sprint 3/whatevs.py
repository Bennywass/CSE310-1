import firebase_admin
import string
from firebase_admin import credentials, firestore

def main():
    
    # Use a service account
    cred = credentials.Certificate("C:/Users/benny/BYU-I/whatevs-d93b4-firebase-adminsdk-h783g-7183c793f4.json")

    # Initialize the app with the service account
    app = firebase_admin.initialize_app(cred)

    # Initialize Firestore DB
    db = firestore.client()

    # Create a playlist
    create_playlist(db)

    # Delete old playlist
    delete_collection(db.collection("Playlist-Old"), batch_size=10)

    # Create a new playlist
    create_new_playlist(db)

    # Read all current data
    read_database(db)

    # Delete one song
    delet_song(db)

    read_database(db)

    # Delete the new playlist
    delete_collection(db.collection("Playlist"), batch_size=10)

# Song class
class Song:

    # Attributes of a song
    def __init__(self, artist, genre):
        self.artist = artist
        self.genre = genre

    def to_dict(self):
        return {
            "artist": self.artist,
            "genre": self.genre
        }

    def __repr__(self):
        return f"Song(artist={self.artist}, genre={self.genre})"

def create_playlist(db):
    # Make an old playlist to be deleted    
    song = Song(artist="Carly Rae Jepsen", genre="pop")
    db.collection("Playlist-Old").document("Call Me Maybe").set(song.to_dict())

    song = Song(artist="Owl City", genre="pop")
    db.collection("Playlist-Old").document("Fireflies").set(song.to_dict())

    song = Song(artist="Green Day", genre="punk")
    db.collection("Playlist-Old").document("Wake Me Up When September Ends").set(song.to_dict())

    song = {"artist": "ABBA", "genre": "pop"}
    db.collection("Playlist-Old").document("Dancing Queen").set(song)

# Delete a song
def delet_song(db):
    db.collection("Playlist").document("Fireflies").delete()

# define function to delete entire collection
def delete_collection(coll_ref, batch_size):
    docs = coll_ref.limit(batch_size).stream()
    deleted = 0

    for doc in docs:
        print(f"Deleting doc {doc.id} => {doc.to_dict()}")
        doc.reference.delete()
        deleted += 1

    if deleted >= batch_size:
        return delete_collection(coll_ref, batch_size)


# Create a new playlist
def create_new_playlist(db):
    song = Song(artist="Carly Rae Jepsen", genre="pop")
    db.collection("Playlist").document("Call Me Maybe").set(song.to_dict())

    song = Song(artist="Owl City", genre="pop")
    db.collection("Playlist").document("Fireflies").set(song.to_dict())

    song = Song(artist="Green Day", genre="punk")
    db.collection("Playlist").document("Wake Me Up When September Ends").set(song.to_dict())

    song = {"artist": "ABBA", "genre": "pop"}
    db.collection("Playlist").document("Dancing Queen").set(song)

# Make a way to read all current data
def read_database(db):
    docs = db.collection("Playlist").stream()

    for doc in docs:
        print(f"{doc.id} => {doc.to_dict()}")

main()