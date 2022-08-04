import transformations from './transformations.json';
import { Node, parse, } from 'acorn';
import { findNodeAt, FindPredicate } from 'acorn-walk';
import fs from 'fs';
import path from 'path';
import type { ClassDeclaration, BaseNode, MethodDefinition, Program } from 'estree';
import { generate } from 'astring';

const file = path.resolve(__dirname, '..', 'pkg', 'rust_game.js');
const source = fs.readFileSync(file, 'utf-8');
const tree = parse(source, { ecmaVersion: 'latest', sourceType: 'module' });

for (const [name, classTransfo] of Object.entries(transformations.classDeclarations)) {
    const classDeclaration = findNode(tree, isClassDeclarationFor(name));

    assertDefined(classDeclaration);

    if (classTransfo.extends) {
        classDeclaration.superClass = {
            type: 'Identifier',
            name: classTransfo.extends,
        };

        const constructor = findNode(classDeclaration, isConstructor);

        assertDefined(constructor);

        constructor.value.body.body.unshift({
            type: "ExpressionStatement",
            expression: {
                type: 'CallExpression',
                callee: { type: 'Identifier', name: 'super' },
                arguments: [],
                optional: false
            }
        });

        constructor.value.body.body.pop();

        constructor.value.body.body.push({
            type: "ExpressionStatement",
            expression: {
                type: "AssignmentExpression",
                operator: "=",
                left: {
                    type: "MemberExpression",
                    object: {
                        type: "ThisExpression",
                    },
                    property: {
                        type: "Identifier",
                        name: "ptr"
                    },
                    computed: false,
                    optional: false
                },
                right: {
                    type: "Identifier",
                    name: "ret"
                }
            }
        });

        if (classTransfo.postConstructible) {
            constructor.value.body.body.push({
                type: "ExpressionStatement",
                expression: {
                    type: "CallExpression",
                    callee: {
                        type: "MemberExpression",
                        object: {
                            type: "Identifier",
                            name: "this"
                        },
                        property: {
                            type: "Identifier",
                            name: "__postConstruct"
                        },
                        computed: false,
                        optional: false
                    },
                    arguments: [
                        {
                            type: "ThisExpression",
                        }
                    ],
                    optional: false
                }
            });
        }
    }
}

function findNode<T extends Node>(ast: Node, predicate: (node: Node) => node is T): T | null {
    const found = findNodeAt(tree, undefined, undefined, (_, node) => predicate(node));

    if (found) {
        return found.node as T;
    }

    return null;
}

function assertDefined<T>(val: T | null | undefined): asserts val is T {
    if (val === null || typeof val === 'undefined') {
        throw new Error('expected defined');
    }
}

function isClassDeclarationFor(name: string) {
    return <T extends BaseNode>(node: T): node is T & ClassDeclaration => {
        return isClassDeclaration(node) && node.id?.name === name;
    };
}

function isConstructor<T extends BaseNode>(node: T): node is T & MethodDefinition & { kind: 'constructor' } {
    return isMethodDefinition(node) && node.kind === 'constructor';
}


function isMethodDefinition<T extends BaseNode>(node: T): node is T & MethodDefinition {
    return node.type === 'MethodDefinition';
}

function isClassDeclaration<T extends BaseNode>(node: T): node is T & ClassDeclaration {
    return node.type === 'ClassDeclaration';
}

const dest = generate(tree);
fs.writeFileSync(file, dest, 'utf-8');
