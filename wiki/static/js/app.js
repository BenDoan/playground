var app = angular.module('wiki', ['ui.bootstrap', 'ngRoute', 'wikiServices']).
    config(['$routeProvider', function($routeProvider){
    $routeProvider.
        when('/', {
            templateUrl: 'partials/index.html',
            controller: 'MainCtrl'
        }).
        when('/article/:title', {
            templateUrl: 'partials/article.html',
            controller: 'ArticleCtrl'
        }).
        otherwise({
            redirectTo: '/'
        });
}]);

app.controller("MainCtrl", ['$scope', 'Articles', function($scope, Articles){
    $scope.name = "World";
    $scope.articles = Articles.query();
}]);

app.controller("ArticleCtrl", ['$scope', '$http', '$routeParams', '$location',
    function($scope, $http, $routeParams, $location){
        title = $routeParams.title;
        $scope.title = title;

        $http({method: 'GET', url: '/article?title='+title}).
        success(function(data, status, headers, config) {
            $scope.body = data;
        }).
        error(function(data, status, headers, config) {
            $scope.body = "Couldn't find"
        });
    }]);
