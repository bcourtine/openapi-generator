//
// User.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation


public struct User: Codable { 


    public var id: Int64?
    public var username: String?
    public var firstName: String?
    public var lastName: String?
    public var email: String?
    public var password: String?
    public var phone: String?
    /** User Status */
    public var userStatus: Int?

    public init(id: Int64?, username: String?, firstName: String?, lastName: String?, email: String?, password: String?, phone: String?, userStatus: Int?) {
        self.id = id
        self.username = username
        self.firstName = firstName
        self.lastName = lastName
        self.email = email
        self.password = password
        self.phone = phone
        self.userStatus = userStatus
    }

}
