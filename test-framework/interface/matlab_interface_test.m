shared_memory_functions_directory = which('set_shared_memory_path');
lib_path = fullfile(fileparts(shared_memory_functions_directory),'..','test-framework','interface','matlab_interface_test_data');

matlab_types={'uint8','uint16','uint32','uint64','int8','int16','int32','int64','single','double'};
testCase = matlab.unittest.TestCase.forInteractiveUse;
empty_path_error="SharedMemory:emptyLibraryPathError";
try
    delete_shared_memory();
catch
    % file does not exists
end
unload_shared_memory_library();

verifyError(testCase,@() get_shared_memory_data_type(),     empty_path_error);
verifyError(testCase,@() get_shared_memory_rank(),          empty_path_error);
verifyError(testCase,@() get_shared_memory_flatten_length(),empty_path_error);
verifyError(testCase,@() get_shared_memory_dimensions(),    empty_path_error);
verifyError(testCase,@() get_shared_memory_flatten_data(),  empty_path_error);
verifyError(testCase,@() get_shared_memory_data(),          empty_path_error);

set_shared_memory_path(lib_path);

for i = 1:length(matlab_types)
    type=matlab_types{i};
    generate_case_fixed(testCase,type, [5 1]);
    generate_case_fixed(testCase,type, [6 4]);
    generate_case_fixed(testCase,type, [3 5 7]);

    generate_case_random(testCase,type, 2);
    generate_case_random(testCase,type, 5);
    generate_case_random(testCase,type, 7);
end

delete_shared_memory();

function generate_case_fixed(testCase,type,size)
    data = cast(randi(10,size,'uint8'),type);
    set_shared_memory_data(data);

    verifyEqual(testCase,get_shared_memory_data_type(),type);
    verifyEqual(testCase,get_shared_memory_rank(),length(size));
    verifyEqual(testCase,get_shared_memory_dimensions(),uint64(size));
    verifyEqual(testCase,get_shared_memory_flatten_length(),prod(size));
    verifyEqual(testCase,get_shared_memory_data(),data);
end


function generate_case_random(testCase,type,rank)
    size = randi([2 5],1,rank,'uint8');
    
    data = cast(randi(10,size,'uint8'),type);
    set_shared_memory_data(data);

    verifyEqual(testCase,get_shared_memory_data_type(),type);
    verifyEqual(testCase,get_shared_memory_rank(),rank);
    verifyEqual(testCase,get_shared_memory_dimensions(),uint64(size));
    verifyEqual(testCase,get_shared_memory_flatten_length(),prod(size));
    verifyEqual(testCase,get_shared_memory_data(),data);

end