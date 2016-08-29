initSidebarItems({"fn":[["client_issued_deletes","Return the amount of calls that were done to `delete`"],["client_issued_gets","Return the amount of calls that were done to `get`"],["client_issued_posts","Return the amount of calls that were done to `post`"],["client_issued_puts","Return the amount of calls that were done to `put`"],["create_account","Create a registered client. This or any one of the other companion functions to get a client must be called before initiating any operation allowed by this crate. `client_handle` is a pointer to a pointer and must point to a valid pointer not junk, else the consequences are undefined."],["create_unregistered_client","Create an unregistered client. This or any one of the other companion functions to get a client must be called before initiating any operation allowed by this crate."],["drop_client","Discard and clean up the previously allocated client. Use this only if the client is obtained from one of the client obtainment functions in this crate (`create_account`, `log_in`, `create_unregistered_client`). Using `client_handle` after a call to this functions is undefined behaviour."],["drop_vector","Drop the vector returned as a result of the execute_for_content fn"],["execute","General function that can be invoked for performing a API specific operation that will return only result to indicate whether the operation was successful or not. This function would only perform the operation and return 0 or error code c_payload refers to the JSON payload that can be passed as a JSON string. The JSON string should have keys module, action, app_root_dir_key, safe_drive_dir_key, safe_drive_access and data. `data` refers to API specific payload."],["execute_for_content","General function that can be invoked for getting data as a resut for an operation. The function return a pointer to a U8 vecotr. The size of the U8 vector and its capacity is written to the out params c_size & c_capacity. The size and capcity would be required for droping the vector The result of the execution is returned in the c_result out param"],["get_account_info","Get data from the network. This is non-blocking. `data_stored` means number of chunks Put. `space_available` means number of chunks which can still be Put."],["get_app_dir_key","Returns key size"],["get_nfs_writer","Obtain NFS writer handle for writing data to a file in streaming mode"],["get_safe_drive_key","Returns Key as base64 string"],["init_logging","This function should be called to enable logging to a file"],["log_in","Log into a registered client. This or any one of the other companion functions to get a client must be called before initiating any operation allowed by this crate. `client_handle` is a pointer to a pointer and must point to a valid pointer not junk, else the consequences are undefined."],["nfs_create_file","Create a file and return a writer for it."],["nfs_stream_close","Closes the NFS Writer handle"],["nfs_stream_write","Write data to the Network using the NFS Writer handle"],["output_log_path","This function should be called to find where log file will be created. It will additionally create an empty log file in the path in the deduced location and will return the file name along with complete path to it."],["register_network_event_observer","Register an observer to network events like Connected, Disconnected etc. as provided by the core module"]],"mod":[["errors","Errors thrown by the FFI operations"]],"struct":[["FfiHandle","A handle, passed through the FFI."],["ParameterPacket","ParameterPacket acts as a holder for the standard parameters that would be needed for performing operations across the modules like nfs and dns"]],"trait":[["Action","ICommand trait"]],"type":[["ResponseType","ResponseType specifies the standard Response that is to be expected from the Action trait"]]});