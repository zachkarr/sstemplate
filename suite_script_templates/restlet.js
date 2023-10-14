/**
 *@NApiVersion 2.1
 *@NScriptType Restlet
 *@NModuleScope SameAccount
 */
define(["N/record", "N/error"], function (record, error) {
  // Get a standard NetSuite record
  function _get(context) {
    return JSON.stringify(
      record.load({
        type: context.recordtype,
        id: context.id,
      })
    );
  }
  // Delete a standard NetSuite record
  function _delete(context) {
    record.delete({
      type: context.recordtype,
      id: context.id,
    });
    return String(context.id);
  }
  // Create a NetSuite record from request params
  function post(context) {
    var rec = record.create({
      type: context.recordtype,
    });
    for (var fldName in context) {
      if (context.hasOwnProperty(fldName))
        if (fldName !== "recordtype") rec.setValue(fldName, context[fldName]);
    }
    var recordId = rec.save();
    return String(recordId);
  }
  // Upsert a NetSuite record from request param
  function put(context) {
    var rec = record.load({
      type: context.recordtype,
      id: context.id,
    });
    for (var fldName in context) {
      if (context.hasOwnProperty(fldName))
        if (fldName !== "recordtype" && fldName !== "id") rec.setValue(fldName, context[fldName]);
    }
    rec.save();
    return JSON.stringify(rec);
  }
  return {
    get: _get,
    delete: _delete,
    post: post,
    put: put,
  };
});
