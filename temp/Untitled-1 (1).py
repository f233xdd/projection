class A:
    def __init__(self,name,place):
        self.name=name
        self.place=place
        print(name,place)
s=input("Do you want to play('yes or no')")
if s=="yes":
      name = input("what is your name")
      place = input("where are you from")
      a = A(name,place)
      list=[]
      list.append((a.name,a.place))
      print(list)
elif s == "no":
     print("ok")
else:
     print("I said yes or no ")
c=input("again?(yes or no)")
while c != "no":
     # do something

