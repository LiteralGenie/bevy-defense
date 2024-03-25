screen -dmS gui
screen -S gui -X stuff "cd /app/gui^M" 
screen -S gui -X stuff "npm run dev^M"
