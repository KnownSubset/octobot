var messages = require('../lib/messages');

describe("messages", function() {

    function makeUser(loginName) {
        return {
            login: loginName
        }
    }

    describe("assignees", function() {
        it("should be empty if no PR given", function() {
            expect(messages.assignees(null, null)).toEqual([]);
        });

        it("should be return a list of assignees", function() {
            expect(messages.assignees({
                assignees: [
                    makeUser('a'), makeUser('b'),
                ]
            }, null)).toEqual(['@a', '@b']);
        });

        it("should accept a single assignee", function() {
            expect(messages.assignees({
                assignee: makeUser('a'),
            }, null)).toEqual(['@a']);
         });
    });

    describe("sendToAll", function() {

        var slack;
        var attachments;
        beforeEach(function() {
            slack = jasmine.createSpyObj('slack', ['send']);
            attachments = ['a', 'b'];
        });

       it("should always send messages to channel", function () {
            messages.sendToAll(slack, 'hello', attachments, {}, null);
            expect(slack.send).toHaveBeenCalledWith({
                text: 'hello',
                attachments: attachments,
            });
        });

        it("should always include the repo URL in the message", function () {
            messages.sendToAll(slack, 'hello', attachments, {}, {  html_url: 'http://haha', full_name: 'TheRepo' });
            expect(slack.send).toHaveBeenCalledWith({
                text: 'hello (<http://haha|TheRepo>)',
                attachments: attachments,
            });
        });

        it("should send messages to assignees if present", function () {
            messages.sendToAll(slack, 'hello', attachments, {
                assignees: [
                   makeUser('a'), makeUser('b'),
                ],
            }, null);
            expect(slack.send).toHaveBeenCalledWith({
                text: 'hello',
                attachments: attachments,
                channel: '@a'
            });
            expect(slack.send).toHaveBeenCalledWith({
                text: 'hello',
                attachments: attachments,
                channel: '@b'
            });
        });

        it("should send messages to item owner if present", function () {
            messages.sendToAll(slack, 'hello', attachments, { user: makeUser('bob') }, null);
            expect(slack.send).toHaveBeenCalledWith({
                text: 'hello',
                attachments: attachments,
                channel: '@bob'
            });
        });
    });
});

