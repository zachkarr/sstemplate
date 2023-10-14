/**
 * @NApiVersion 2.1
 * @NScriptType ClientScript
 * @NModuleScope SameAccount
 */
define(["N/record", "N/runtime", "N/search"], function (record, runtime, search) {
  function lineInit(context) {
    return;
  }

  function pageInit(context) {
    return;
  }

  function postSourcing(context) {
    return;
  }

  function saveRecord(context) {
    return true; //Return true if you want to continue saving the record.
  }

  function sublistChanged(context) {}

  function validateDelete(context) {
    return true; //Return true if the line deletion is valid.
  }

  function validateField(context) {
    return true; //Return true to continue with the change.
  }

  function validateInsert(context) {
    return true; //Return true if the line insertion is valid.
  }

  function validateLine(context) {
    return true; //Return true if the line is valid.
  }

  function fieldChanged(context) {
    return;
  }
});

return {
  lineInit: lineInit,
  pageInit: pageInit,
  postSourcing: postSourcing,
  saveRecord: saveRecord,
  sublistChanged: sublistChanged,
  validateDelete: validateDelete,
  validateField: validateField,
  validateInsert: validateInsert,
  validateLine: validateLine,
  fieldChanged: fieldChanged,
};
