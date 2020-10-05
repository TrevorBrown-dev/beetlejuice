import Discord from "discord.js";
import axios from "axios";  

const client = new Discord.Client();

client.once('ready', () => {
    console.log("Ready!");
    client.user.setActivity("Cruisin' to Bruise")
    //client.user.setAvatar("./beetlejuice.jpg")  incase avatar is gone
});

client.once('reconnecting', () => {
    console.log('Reconnecting!');
});

client.once('disconnect', () => {
    console.log('Disconnect!');
});

client.on("message", (message) => {
    let toggle = false;
    console.log(toggle);
    let args = message.content.split(" ");
    if(toggle === false && args[0] === '!secret'){
        message.delete();
        toggle = true;
        console.log(toggle);
    }else if(toggle === true && args[0] === '!secret'){
        message.delete();
        toggle = false;
        console.log(toggle);
    }    
    const user = message.mentions.members.first();
    //console.log(user);
    //console.log(message.author.id);
    axios.get(`https://www.purgomalum.com/service/containsprofanity?text=${message.content}`).then((response) =>{
        if(response.data){
            if(!message.author.bot){
                if(!(message.content.includes("!shit") || message.content.includes("!shitpost"))){
                    message.delete();
                    message.channel.send(`That word is a no go ${message.author}`);
                }
                //Intent: To troll trevor 760676060550398002
                if(message.author.id.includes(219853415184990208) && toggle === true){
                    console.log(working);
                    message.delete();
                    message.channel.send(`That word is a no go ${message.author}`)
                }
            }
        }
    })

    switch (args[0]){

        case "!kick":
            if(user){
                user.kick().then(() =>{
                    message.reply(`Successfully fucked ${user}`)
                }).catch(err => {
                    message.reply(" I was a bad boi and not able to kick");
                    console.log(err);
                })
            }else{
                message.reply("You need to specify a person in order to kick a member form the server")
            }
        break;

        case "!ban":
            if(user){
                user.ban().then(() =>{
                    message.reply(`Successfully superfucked ${user}`)
                }).catch(err => {
                    message.reply(" I was a bad boi and not able to ban");
                    console.log(err);
                })
            }else{
                message.reply("You need to specify a person in order to ban a member form the server")
            }
        break;
    }
})

client.login(process.env.BOT_TOKEN);