def choices(r1,r2,r3):
 while True:
   choices = int(input("\nEnter The Number Crossponding To Your Choice :"))
   if choices == 1 :
     print(f"You Chose {r1}")
     break
   elif choices == 2 :
      print(f"You Chose {r2}")
      break
   elif choices == 3 :
     print(f"You Chose {r3}")
     break
   else:
     print("Invalid Input.Enter Again..")
 return r1 or r2 or r3

  

template = ('''
On a {adjective1} night, deep in the {place}, a {adjective2} figure known as the Chainsaw Man emerged from the {noun1}. 
His {adjective3} chainsaw roared to life, echoing through the {noun2}. 
The air was thick with the scent of {smell} as he hunted for his next {noun3}. 
With each step, the ground trembled, and the {adjective4} moon cast a {adjective5} glow on his {noun4}. 
'''
)

print(template)

while True:
  #pahla adjective ka liya...........
  a = "Haunting" 
  b = "Sinister"
  c = "Horrific"
  print("\nChose An Adjective From The Given Options.")
  print(f"1. {a}")
  print(f"2. {b}")
  print(f"3. {c}")
  value = choices(a,b,c)

      # ab place ka liya kara ga...........

  a1 = "Ghostly Cemetery" 
  b1 = "Abandoned Asylum"
  c1 = " Demonic Labyrinth"
  print("\nChose A Place From The Given Options.")
  print(f"1. {a1}")
  print(f"2. {b1}")
  print(f"3. {c1}")
  value1 = choices(a1,b1,c1) 

      #ab dusra adjective ka liya kara ga........

  a2 = "BloodCurdling" 
  b2 = "Terrifying"
  c2 = "Nightmarish"
  print("\nChose An Adjective From The Given Options.")
  print(f"1. {a2}")
  print(f"2. {b2}")
  print(f"3. {c2}")
  value2 = choices(a2,b2,c2)
      
  # ab noun ka liya kara ga.......\

  a3 = "UnderWorld" 
  b3 = "GraveYard"
  c3 = "Abyss"
  print("\nChose A Noun From The Given Options.")
  print(f"1. {a3}")
  print(f"2. {b3}")
  print(f"3. {c3}")
  value3 = choices(a3,b3,c3)    
      # ab 3 adjective ka liya ..........

  a4 = "BloodThirsty" 
  b4 = "Unholy"
  c4 = "Diabolical"
  print("\nChose A Adjective From The Given Options.")
  print(f"1. {a4}")
  print(f"2. {b4}")
  print(f"3. {c4}")
  value4 = choices(a4,b4,c4)    
  # ab dusra noun ka liya..........

  a5 = "Labyrinth" 
  b5 = "Abyss"
  c5 = "Graveyard"
  print("\nChose A Noun From The Given Options.")
  print(f"1. {a5}")
  print(f"2. {b5}")
  print(f"3. {c5}")
  value5 = choices(a5,b5,c5)

  #ab smell ka liya .........

  a6 = "Rotting Flesh" 
  b6 = "Death"
  c6 = "Burnt Bones"
  print("\nChose A Smell From The Given Options.")
  print(f"1. {a6}")
  print(f"2. {b6}")
  print(f"3. {c6}")
  value6 = choices(a6,b6,c6)

  # ab 3 noun ka liya..........

  a7 = "Soul" 
  b7 = "Victim"
  c7 = "Sacrifice"
  print("\nChose A Noun From The Given Options.")
  print(f"1. {a7}")
  print(f"2. {b7}")
  print(f"3. {c7}")
  value7 = choices(a7,b7,c7)

  # ab 4 adjective ki barri ha......

  a8 = "Blood-Red" 
  b8 = "Gloomy"
  c8 = "Phantom"
  print("\nChose A Adjective From The Given Options.")
  print(f"1. {a8}")
  print(f"2. {b8}")
  print(f"3. {c8}")
  value8 = choices(a8,b8,c8)

  # ab 5 adjective ki barri ha....

  a9 = "Ghoulish" 
  b9 = "Menacing"
  c9 = "Wraithlike"
  print("\nChose An Adjective From The Given Options.")
  print(f"1. {a9}")
  print(f"2. {b9}")
  print(f"3. {c9}")
  value9 = choices(a9,b9,c9)

  # ab 4 noun ki barri ha..........

  a10 = "Corpse" 
  b10 = "Face"
  c10 = "Presence"
  print("\nChose a Noun From The Given Options.")
  print(f"1. {a10}")
  print(f"2. {b10}")
  print(f"3. {c10}")
  value10 = choices(a10,b10,c10)

  # pahla testing karla .............

  story = template
  story = story.replace("{adjective1}", str(value))
  story = story.replace("{place}", str(value1))
  story = story.replace("{adjective2}", str(value2))
  story = story.replace("{noun1}", str(value3))
  story = story.replace("{adjective3}", str(value4))
  story = story.replace("{noun2}", str(value5))
  story = story.replace("{smell}", str(value6))
  story = story.replace("{noun3}", str(value7))
  story = story.replace("{adjective4}", str(value8))
  story = story.replace("{adjective5}", str(value9))
  story = story.replace("{noun4}", str(value10))

  # khatam tata bye bye...........

  print("\nHere Yours Mad Libs Story ...")
  print(f"\n {story} ")

  # dubara khelna ha bahi toh khel la

  again = str(input("\nDo You Want To Play Again (y/n) :"))
  if again != 'y':
    break 









