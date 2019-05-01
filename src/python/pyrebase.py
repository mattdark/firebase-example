import json
from firebase import Firebase
def read_data(self):
    config = {
        "apiKey": "AIzaSyD_mMNY7MH02I5jnEmT9XOTQkerWjFx2TY",
        "authDomain": "reveal-gallery.firebaseapp.com",
        "databaseURL": "https://reveal-gallery.firebaseio.com",
        "projectId": "reveal-gallery",
        "storageBucket": "reveal-gallery.appspot.com",
        "messagingSenderId": "386555259289"
    }
    firebase = Firebase(config)

    slide = list()
    db = firebase.database()
    all_slides = db.child("slides").get()
    for x in all_slides.each():
        slide.append(x.val())
    s = json.dumps(slide)
    return s
