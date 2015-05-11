var wikiServices =
    angular.module('wikiServices', ['ngResource']);

wikiServices.factory('Article', ['$resource',
    function($resource){
        return $resource('/articles', {}, {
            query: {method: 'GET'}, isArray:true)
    }]);
}
