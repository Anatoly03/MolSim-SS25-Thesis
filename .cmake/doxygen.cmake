find_package(Doxygen)

if(DOXYGEN_FOUND)
    message (STATUS "Doxygen Version: ${DOXYGEN_VERSION}")

    include_directories(${DOXYGEN_INCLUDE_DIRS})

    target_link_libraries(MolSim PRIVATE ${DOXYGEN_LIBRARIES})
    target_link_libraries(MolSimCore PRIVATE ${DOXYGEN_LIBRARIES})
    target_link_libraries(MolSimTest PRIVATE ${DOXYGEN_LIBRARIES})

    add_custom_target(docs COMMAND doxygen ../../Doxyfile)
endif()