angular.module('eMeatApp', []).controller('eMeatController', function($scope, $sce, $http) {
    var eMeat = this;

    eMeat.attendees = [];

    eMeat.availableAdds = [
        {id: 0, name: '0'},
        {id: 1, name: '1'},
        {id: 2, name: '2'},
        {id: 3, name: '3'},
        {id: 4, name: '4'},
    ];
    eMeat.attPluss = {id: '0', name: '0'}

    $scope.refresh = function() {
        eMeat.attendees = [];
        eMeat.attendee_count = 0;
        $http.get('api/get_attendees').then(function(response) {
            var rsp = response.data;
            eMeat.attendee_count = Object.keys(rsp).length;
            angular.forEach(rsp, function(v, k) {
                console.log(v["name"])
                console.log(v["additional"])
                eMeat.attendees.push({name: v["name"], pluss: v["additional"]});
                eMeat.attendee_count += v["additional"];
            });
        });
    }

    $scope.refresh();

    eMeat.addAttendee = function() {
        var data = {"name": eMeat.attName, "additional": parseInt(eMeat.attPluss.id) };
        console.log(data)
        var config = {};
        $http.post('api/add_attendee', data);
        eMeat.attName = "";
        eMeat.attPluss = {id: '0', name: '0'}
        $scope.refresh();
    };


    eMeat.title = "Me no know";
    eMeat.description = "";
    $scope.description = $sce.trustAsHtml(eMeat.description);
    eMeat.add_description = function() {
        $http.get('api/get_description').then(function(resp) {
            var title = resp.data.title;
            var description = resp.data.description;
            eMeat.title = title;
            eMeat.description = description;
            $scope.description = $sce.trustAsHtml(eMeat.description);
        });
    };

    eMeat.add_description();

});
