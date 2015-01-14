Votes = new Mongo.Collection("votes");

if (Meteor.isClient) {
    Template.body.helpers({
        votes: function(){
            return Votes.find({}, {sort: {createdAt: -1}}).map(
                    function(obj, index){
                        console.log(index);
                        obj.index = index;
                        return obj;
                    });
        }
    });

    Template.body.events({
        "submit .new-vote": function (event){
            var question = event.target.question.value;
            var options = event.target.options.value.split(",");

            Votes.insert({
                question: question,
                options: options,
                createdAt: new Date(),
                owner: Meteor.userId(),
                username: Meteor.user().username,
                votes: {}
            });

            event.target.question.value = "";
            event.target.options.value = "";

            return false;
        },
        "submit .vote": function (event){
            var voteoption = event.target.voteoption.value;

            return false;
        }
    });

    Template.vote.events({
        "click .delete": function(){
            Votes.remove(this._id);
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
