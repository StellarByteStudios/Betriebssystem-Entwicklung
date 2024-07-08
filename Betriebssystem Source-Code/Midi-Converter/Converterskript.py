import mido
from mido import MidiFile



def main():

    # Pfad zum Midi-File
    pathToMedi = "Midifiles/intro.mid"
    # Pfad zur Ausgabe-Datei
    pathToMethod = "MidiOutput/intro.txt"

    # Speed-Faktor
    #bpm = 100
    speedFactor = 700 #* bpm/120

    # Midi-File einlesen
    midifile = MidiFile(pathToMedi)
    # Midi Convertieren
    notes_and_rests = midi_to_frequencies_durations_and_rests(midifile, speedFactor)

    # Frequenzen in Methoden-Format schreiben
    formatedMethod = "pub fn intro() {\n"

    delayskip = 0
    for item in notes_and_rests:
        # Schauen ob es eine Pause ist
        if item[0] == 'rest':
            # Ist die Pause zu kurz wird sie an nächste Note angehangen
            if int(item[1]) < 10:
                #delayskip = int(item[1]) 
                formatedMethod += f"    delay({10});\n"
                delayskip = 0
            if int(item[1]) >= 10:
                formatedMethod += f"    delay({int(item[1])});\n"
        else:
            if int(item[1]) >= 20:
                formatedMethod += f"    play({item[0]:.2f}, {int(item[1] + delayskip)});\n"
                delayskip = 0
            else:
                formatedMethod += f"    play({item[0]:.2f}, {int(20 + delayskip) });\n"
                delayskip = 0

    
    formatedMethod +="}\n"


    # Neue Methodenaufrufe als Datei speichern
    with open(pathToMethod, 'w') as file:
        file.write(formatedMethod)
    
    print(f"{formatedMethod}")
    #for item in notes_and_rests:
    #    print(item)
    #print(f"{notes_and_rests}")

    """
    # Midi-File einlesen
    midifile = MidiFile(pathToMedi)
    # Midi Convertieren
    notes_and_rests = midi_to_frequencies_durations_and_rests(midifile)

    # Frequenzen in Methoden-Format schreiben
    formatedMethod = "pub fn starwars_imperial() {\n"

    for item in notes_and_rests:
        # Schauen ob es eine Pause ist
        if item[0] == 'rest':
            # Ist die Pause zu kurz wird sie weg gelassen
            if int(item[1]) >= 10:
                formatedMethod += f"    delay({int(item[1]-10)});\n"
        else:
            if int(item[1]) >= 10:
                formatedMethod += f"    play({item[0]:.2f}, {int(item[1]-10)});\n"
            else:
                formatedMethod += f"    play({item[0]:.2f}, 10);\n"

    
    formatedMethod +="}\n"


    """




    return




# Define the function to convert MIDI note numbers to frequencies
def midi_note_to_freq(note):
    # Standard A440 tuning
    A4 = 440.0
    return A4 * 2**((note - 69) / 12.0)



# Define the function to convert ticks to milliseconds
def ticks_to_ms(ticks, tempo, ticks_per_beat, speedFactor=1):
    # Calculate time per tick in microseconds
    us_per_tick = tempo / ticks_per_beat
    # Convert microseconds to milliseconds
    ms_per_tick = us_per_tick / 1000
    return ticks * ms_per_tick * speedFactor




def midi_to_freq_duration(mid):
    ticks_per_beat = mid.ticks_per_beat

    print(f"ticks_per_beat: {ticks_per_beat}")
    
    freq_duration_list = []
    
    # Default tempo is 500000 microseconds per beat (120 BPM)
    tempo = 500000
    
    for track in mid.tracks:
        current_time = 0
        for msg in track:
            current_time += msg.time
            if msg.type == 'set_tempo':
                tempo = msg.tempo
            if msg.type == 'note_on' and msg.velocity > 0:
                note_on_time = current_time
                note = msg.note
                frequency = midi_note_to_freq(note)
            if msg.type == 'note_off' or (msg.type == 'note_on' and msg.velocity == 0):
                note_off_time = current_time
                duration_ticks = note_off_time - note_on_time
                duration_ms = ticks_to_ms(duration_ticks, tempo, ticks_per_beat)
                freq_duration_list.append((frequency, duration_ms))
    
    return freq_duration_list




def midi_to_frequencies_durations_and_rests(midi, speedFactor=1):
    notes_and_rests = []
    current_time = 0
    ongoing_notes = {}
    tempo = 500000  # Default tempo in microseconds per beat (120 BPM)
    ticks_per_beat = midi.ticks_per_beat

    last_note_end_time = 0

    # Iterate through all messages in the MIDI file
    for msg in midi:
        current_time += msg.time

        if msg.type == 'set_tempo':
            tempo = msg.tempo

        if msg.type == 'note_on' and msg.velocity > 0:
            # Note on event with non-zero velocity (start of the note)
            if last_note_end_time and (current_time > last_note_end_time):
                rest_duration_ticks = current_time - last_note_end_time
                rest_duration_ms = ticks_to_ms(rest_duration_ticks, tempo, ticks_per_beat, speedFactor)
                notes_and_rests.append(('rest', rest_duration_ms))
            ongoing_notes[msg.note] = current_time

        elif msg.type == 'note_off' or (msg.type == 'note_on' and msg.velocity == 0):
            # Note off event or note on event with zero velocity (end of the note)
            if msg.note in ongoing_notes:
                start_time = ongoing_notes.pop(msg.note)
                duration_ticks = current_time - start_time
                duration_ms = ticks_to_ms(duration_ticks, tempo, ticks_per_beat, speedFactor)
                frequency = midi_note_to_freq(msg.note)
                notes_and_rests.append((frequency, duration_ms))
                last_note_end_time = current_time

    return notes_and_rests








if __name__ == "__main__":
    main()