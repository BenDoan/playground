var wikiServices =
    angular.module('wikiServices', ['ngResource']);

wikiServices.factory('Articles', ['$resource',
    function($resource){
        return $resource('/articles', {}, {
            query: {method: 'GET', isArray:false}}
        )
    }
])

wikiServices.factory('Article', ['$resource',
    function($resource){
        return $resource('/article?title=:title', {title: '@title'}, {
            query: {method: 'GET', isArray:false}}
        )
    }
])
