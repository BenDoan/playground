var app = angular.module('wiki', ['ui.bootstrap', 'ngRoute', 'wikiServices']).
    config(['$routeProvider', function($routeProvider){
    $routeProvider.
        when('/', {
            templateUrl: 'partials/index.html',
            controller: 'MainCtrl'
        }).
        when('/article/:title/edit', {
            templateUrl: 'partials/edit.html',
            controller: 'ArticleCtrl'
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

app.controller("ArticleCtrl", ['$scope', '$http', '$routeParams', '$location', '$sce',
    function($scope, $http, $routeParams, $location, $sce){
        title = $routeParams.title;
        $scope.title = title;

        $http({method: 'GET', url: '/article?title='+title}).
        success(function(data, status, headers, config) {
            console.log(data.body)
            $scope.body = data;
            $scope.article = {
                title: title,
                body: data,
                html_body: data
            }
        }).
        error(function(data, status, headers, config) {
            $scope.body = "Couldn't find"
            $scope.article = {
                title: title,
                body: ""
            }
        });

        $scope.update = function(article){
            $http({
                method: 'put',
                url: '/article',
                params: {title: article.title, body: article.body}
            }).
            success(function(data, status, headers, config) {
                console.log("success");
            }).
            error(function(data, status, headers, config) {
                console.log("failure");
            });

            $location.path("/article/"+title);
        };

        $scope.getHtmlBody = function(){
            return $sce.trustAsHtml($scope.body);
        }
    }]);
