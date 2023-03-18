import time
from tkinter import *
import math
#initialize window
Window=Tk()
ScreenWidth=Window.winfo_screenwidth()
ScreenHeight=Window.winfo_screenheight()
Window.geometry("%dx%d"%(ScreenWidth,ScreenHeight))
Window.title("Terra Game Engine")
BC=Canvas(Window, height=ScreenHeight, width=ScreenWidth, bg="black")
#initialize gameloop
GameRunning=True
timenow=time.time()
lasttime=timenow
deltatime=0
#initialize gameworld
#x=left to right,y=how far from you, z=height
worldc={
    "wall1":[
        [5,10,0],
       [10,10,0],
        [5,10,20]
        ]#,
    #"wall2":[
    #    [5,10,20],
    #    [10,10,20],
    #    [10,10,0]
     #   ]
        }
translated={}
vertexs={}
degreescos=[]
degreessin=[]
for x in range(360):
    pi=math.pi
    #degreescos.append(math.cos(math.radians(x)))
    degreescos.append(math.cos(x/180*pi))
    #degreessin.append(math.sin(math.radians(x)))
    degreessin.append(math.sin(x/180*pi))
playerx=0
playery=0
playerz=20
playera=0
playerua=0
playerfov=200
#movement calls
def left(event):
    global playery
    global playerx
    playery+=(1*degreessin[playera])
    playerx-=(1*degreescos[playera])
    return
def right(event):
    global playery
    global playerx
    playery-=(1*degreessin[playera])
    playerx+=(1*degreescos[playera])    
    return
def up(event):
    global playery
    global playerx
    playery+=(1*degreescos[playera])
    playerx+=(1*degreessin[playera])
    return
def down(event):
    global playery
    global playerx
    playery-=(1*degreescos[playera])
    playerx-=(1*degreessin[playera])
    return
def a (event):
    global playera
    playera-=4
    if playera<0:
        playera+=360
    return
def d (event):
    global playera
    playera+=4
    if playera>359:
        playera-=360
    return
#transform/set relative to us function
def Transform(x,y,z):
    newx=x-playerx
    newy=y-playery
    cos=degreescos[playera]
    sin=degreessin[playera]
    wnewx=newx*cos-newy*sin
    wnewy=newy*cos+newx*sin
    #wnewz=z-playerz
    wnewz=z-playerz+((playerua*wnewy)/32)
    return [wnewx, wnewy, wnewz]
def Project(x,y,z):
    if y==0:
        return
    prx=(x)*playerfov/y+(ScreenWidth/2)
    pry=(z)*playerfov/(-y)+(ScreenHeight/2)
    absx=x-playerx
    absy=y-playery
    absz=z-playerz
    absoluted=math.sqrt((absx)*(absx)+(absy)*(absy)+(absz)*(absz))
    return [math.ceil(prx),math.ceil(pry),absoluted]
#our physics function
def MainUpdate(deltatime):
    Window.bind_all("<Left>",left)
    Window.bind_all("<Right>",right)
    Window.bind_all("<Up>",up)
    Window.bind_all("<Down>",down)
    Window.bind_all("a",a)
    Window.bind_all("d",d)
    #Window.bind_all("w",w)
    #Window.bind_all("s",s)
    return
#our render function
def sortvertex(vertexs,w):
        vertexs[w].sort(key=lambda x:x[0])
        return vertexs[w]
def Render():
    for w in worldc:
        translated[w]=[]
        vertexs[w]=[]
        cl=0
        for c in worldc[w]:
            translated[w].append([])
            vertexs[w].append([])
            x=c[0]
            y=c[1]
            z=c[2]
            translated[w][cl]=(Transform(x,y,z))
            trax=translated[w][cl][0]#this sets position in array
            tray=translated[w][cl][1]#this sets position in array
            traz=translated[w][cl][2]#this sets position in array
            vertex=Project(trax,tray,traz)
            #print(translated[w])
            if vertex is not None:
                vertexs[w][cl]=(vertex)
            cl+=1
    for w in vertexs:
        vertexs[w]=sortvertex(vertexs,w)
        singleslope=1
        dualslope1=1
        dualslope2=1
        startx=int(vertexs[w][0][0])
        endx=int(vertexs[w][2][0])
        starty=int(vertexs[w][0][1])
        endy=int(vertexs[w][0][1])
        midy=int(vertexs[w][1][1])
        midx=int(vertexs[w][1][0])
        try:((endy-midy)/(endx-midx)==0)
        except:
            pass
        else:
            dualslope2=int((endy-midy)/(endx-midx))
        try:((endy-starty)/(midx-startx)==0)
        except:
            pass
        else:
            dualslope1=int((endy-starty)/(midx-startx))
        try:((endy-starty)/(endx-startx)==0)
        except:
            pass
        else:
            singleslope=int((endy-starty)/(endx-startx))
        if vertexs[w][0][0]<0:
            startx=0
        if vertexs[w][2][0]>ScreenWidth:
            endx=ScreenWidth
        for x in range(startx,endx):
            if starty<0:
                starty=0
            if endy>ScreenHeight:
                endy=ScreenHeight
            yrange=[starty,endy]
            yrange.sort()
            #print(vertexs[w])
            #print(yrange)
            for y in range(yrange[0],yrange[1]+1):
                BC.create_line((x,y),(x+1,y+1),fill='red')
            if x<=midx:
                starty+=dualslope1
            else:
                midy+=dualslope2
                endy=midy
            endy+=singleslope
        BC.create_line(vertexs[w][0][0],vertexs[w][0][1],vertexs[w][1][0],vertexs[w][1][1],vertexs[w][2][0],vertexs[w][2][1],fill='grey')
    BC.create_line(ScreenWidth/2-10,ScreenHeight/2,ScreenWidth/2+10,ScreenHeight/2,fill='grey')
    BC.create_line(ScreenWidth/2,ScreenHeight/2-10,ScreenWidth/2,ScreenHeight/2+10,fill='grey')
    BC.pack()
    Window.update_idletasks()
    Window.update()
    return
while (GameRunning):
    BC.delete('all')
    timenow=time.time()
    #GameRunning=False
    deltatime=timenow-lasttime
    lasttime=lasttime+deltatime
    MainUpdate(deltatime)
    Render()




