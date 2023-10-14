/**
 *@NApiVersion 2.1
 *@NScriptType Portlet
 *@NModuleScope SameAccount
 */

define([], function () {
  function render(params) {
    params.portlet.title = "My Portlet";
    var content = "<td><span><b>Hello!!!</b></span></td>";
    params.portlet.html = content;
  }

  return {
    render: render,
  };
});
