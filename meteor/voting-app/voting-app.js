Questions = new Mongo.Collection("questions");

if (Meteor.isClient) {
    Template.body.helpers({
        votes: function(){
            return Questions.find({}, {sort: {createdAt: -1}});
        }
    });

    Template.body.events({
        "submit .new-vote": function (event){
            var question = event.target.question.value;

            var opts = {}
            var options = event.target.options.value.split(",").forEach(
                    function(element, index){
                        opts[element] = 0
                    });


            Questions.insert({
                question: question,
                options: opts,
                createdAt: new Date(),
                owner: Meteor.userId(),
                username: Meteor.user().username
            });

            event.target.question.value = "";
            event.target.options.value = "";

            return false;
        },
        "submit .vote": function (event){
            var voteoption = event.target.voteoption.value;
            search_string = "options." + voteoption;

            var num = Questions.findOne({_id: this._id}).options[voteoption];

            var set = {};
            set[search_string] = num+1;
            Questions.update({_id: this._id}, {$set: set});
            //Questions.update({_id: this._id}, {$set: {"options.asdsa": 17}});
            return false;
        }
    });

    Template.vote.events({
        "click .delete": function(){
            Questions.remove(this._id);
        }
    });

    Accounts.ui.config({
        passwordSignupFields: "USERNAME_ONLY"
    });
}

if (Meteor.isServer) {
  Meteor.startup(function () {
    // code to run on server at startup
  });
}

Handlebars.registerHelper('arrayify',function(obj){
    result = [];
    for (var key in obj) result.push({key:key,value:obj[key]});
    return result;
});
