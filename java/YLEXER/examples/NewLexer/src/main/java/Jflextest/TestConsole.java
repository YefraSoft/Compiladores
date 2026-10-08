package Jflextest;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.StringReader;

// Versión de consola de TestClass: lee líneas de stdin e imprime los tokens.
public class TestConsole {

	public static void main(String args[]) throws IOException {
		BufferedReader in = new BufferedReader(new InputStreamReader(System.in));
		String line;

		while ((line = in.readLine()) != null) {
			NewLexer lexer = new NewLexer(new StringReader(line));

			while (true) {
				Token token = lexer.yylex();
				if (token == null) {
					System.out.println("FIN");
					break;
				}

				switch (token) {
					case ID:
					case numero:
					case variable:
						System.out.println("Token:" + token + " " + lexer.lexeme);
						break;
					case IF:
					case THEN:
					case ELSE:
						System.out.println("Palabra reservada: " + token);
						break;
					default:
						System.out.println("Token:" + token);
						break;
				}
			}
		}
	}
}
