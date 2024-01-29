## ZinZen algorithm

This document contains a high-level overview of the scheduling algorithm.  

### 1) The Calendar is created

... and spans from start to end date.

### 2) The Celendar places occupied Slots by the given Goals

### 3) Simple Goal Activities are calculated

... and applied to the Calendar.

### 3) Budget Goal Activities are calculated

... and applied to the Calendar.

### 4) Based on the Budget's time

... these activities are placed into the open Slots of the Calendar.

### 5) print??? // ToDo



## Why this algorithm?
If you'r not convinced this algorithm is good - please suggest a better one. I'm all ears!  
Here's a short explanation / test case to show that giving priority to least flexible Steps is wrong:     
<img src="/documentation/functional/the-wrong-way-of-scheduling.png" alt="The-wrong-way" width="800"/>  

To make it extra clear, here is the correct way of scheduling:
<img src="/documentation/functional/the-correct-way-of-scheduling.png" alt="The-correct-way" width="800"/>  
