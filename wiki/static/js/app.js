var app = angular.module('wiki', ['ui.bootstrap', 'ngRoute']).
    config(['$routeProvider', function($routeProvider){
    $routeProvider.
        when('/', {
            templateUrl: 'partials/index.html',
            controller: 'MainCtrl'
        }).
        otherwise({
            redirectTo: '/'
        });
}]);

app.controller("MainCtrl", function($scope){
    $scope.name = "World";
});
